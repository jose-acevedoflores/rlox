use std::env;
use std::fs::File;
use std::io;
use std::io::Write;

type IORes = io::Result<usize>;

fn define_ast(out_dir: &String, base_name: &str, types: &Vec<GrammarTy>) -> IORes {
    let p = std::path::PathBuf::from(out_dir);
    let p = p.join(std::path::PathBuf::from(format!(
        "{}.rs",
        base_name.to_lowercase()
    )));

    let mut f = File::create(p).unwrap();

    f.write("use shared::tokens;\n".as_bytes()).unwrap();

    define_enum_type(&mut f, base_name, &types)?;

    define_struct_types(&mut f, base_name, &types)?;

    define_accept_trait(&mut f, base_name, &types)?;

    define_visitor(&mut f, base_name, &types)
}

fn define_accept_trait(file: &mut File, base_name: &str, types: &Vec<GrammarTy>) -> IORes {
    file.write(
        "pub trait Accept<R> {\n    fn accept(&self, visitor: &dyn Visitor<R>) -> R;\n}\n"
            .as_bytes(),
    )?;

    file.write(format!("impl<R> Accept<R> for {} {{\n", base_name).as_bytes())?;
    file.write(
        format!(
            "{:4}fn accept(&self, visitor: &dyn Visitor<R>) -> R {{\n",
            " "
        )
        .as_bytes(),
    )?;
    file.write(format!("{:8}match self {{\n", " ").as_bytes())?;

    for typ in types {
        file.write(
            format!(
                "{:12}{}::{}(a) => visitor.visit_{}_{}(a),\n",
                " ",
                base_name,
                typ.class_name,
                typ.class_name.to_lowercase(),
                base_name.to_lowercase(),
            )
            .as_bytes(),
        )?;
    }

    file.write(format!("{:8}}}\n{:4}}}\n}}\n", " ", " ").as_bytes())
}

fn define_enum_type(file: &mut File, base_name: &str, types: &Vec<GrammarTy>) -> IORes {
    file.write(format!("pub enum {} {{ \n", base_name).as_bytes())?;
    for ty in types {
        file.write(format!("    {}({}),\n", ty.class_name, ty.class_name).as_bytes())?;
    }
    file.write("} \n".as_bytes())
}

fn define_struct_types(file: &mut File, base_name: &str, types: &Vec<GrammarTy>) -> IORes {
    for typ in types {
        let fields = &typ.fields;

        file.write(format!("pub struct {} {{\n", typ.class_name).as_bytes())?;

        for field in fields {
            let ty = ty_map::map_to_rs_type(field.ty, base_name);
            file.write(format!("{:4}pub {}: {},\n", " ", field.name, ty).as_bytes())?;
        }
        file.write("}\n".as_bytes())?;
    }
    file.write("\n".as_bytes())
}

fn define_visitor(file: &mut File, base_name: &str, types: &Vec<GrammarTy>) -> IORes {
    file.write(format!("pub trait Visitor<R> {{\n").as_bytes())?;
    for ty in types {
        file.write(
            format!(
                "{:4}fn visit_{}_{}(&self, {}: &{}) -> R;\n",
                " ",
                ty.class_name.to_lowercase(),
                base_name.to_lowercase(),
                ty.class_name.to_lowercase(),
                ty.class_name,
            )
            .as_bytes(),
        )?;
    }
    file.write("}".as_bytes())
}

mod ty_map {
    pub enum RsT {
        S(&'static str),
        F(String),
    }

    impl std::fmt::Display for RsT {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                RsT::S(val) => write!(f, "{}", val),
                RsT::F(val) => write!(f, "{}", val),
            }
        }
    }

    pub fn map_to_rs_type(ty: &str, base_name: &str) -> RsT {
        if base_name == ty {
            RsT::F(format!("Box<{}>", base_name))
        } else if ty == "Token" {
            RsT::S("tokens::Token")
        } else if ty == "Object" {
            RsT::S("tokens::LiteralValue")
        } else {
            panic!(format!("Unknown type '{}'", ty))
        }
    }
}

struct TypeName<'a> {
    ty: &'a str,
    name: &'a str,
}

struct GrammarTy<'a> {
    class_name: &'a str,
    fields: Vec<TypeName<'a>>,
}

fn parse_types(grammar: &Vec<String>) -> Vec<GrammarTy> {
    let mut vec = vec![];
    for entry in grammar {
        let mut s = entry.split(":");
        let class_name = s.next().unwrap().trim();
        let fields = s.next().unwrap().split(",");
        let fields = fields
            .map(|f| {
                let mut field = f.trim().split(" ");
                let ty = field.next().unwrap().trim();
                let name = field.next().unwrap().trim();
                TypeName { ty, name }
            })
            .collect();
        vec.push(GrammarTy { class_name, fields });
    }

    vec
}

fn parse_args() -> String {
    let mut args: Vec<String> = env::args()
        .skip(1) //skip executable name
        .collect();

    if args.len() != 1 {
        println!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }

    args.pop().unwrap()
}

fn main() {
    let out_dir = parse_args();

    let v = vec![
        "Binary   : Expr left, Token operator, Expr right".to_string(),
        "Grouping : Expr expression".to_string(),
        "Literal  : Object value".to_string(),
        "Unary    : Token operator, Expr right".to_string(),
    ];

    let types = parse_types(&v);

    define_ast(&out_dir, "Expr", &types).unwrap();
}
