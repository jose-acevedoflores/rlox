use std::env;
use std::fs::File;
use std::io::Write;

fn generate_ast() -> String {
    let mut args: Vec<String> = env::args()
        .skip(1) //skip executable name
        .collect();

    if args.len() != 1 {
        println!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }

    args.pop().unwrap()
}

fn define_ast(out_dir: &String, base_name: &str, types: Vec<String>) {
    let p = std::path::PathBuf::from(out_dir);
    let p = p.join(std::path::PathBuf::from(format!(
        "{}.rs",
        base_name.to_lowercase()
    )));

    let mut f = File::create(p).unwrap();

    f.write("use shared::tokens;\n".as_bytes()).unwrap();

    define_enum_type(&mut f, base_name, &types);

    define_struct_types(&mut f, base_name, &types);

    define_accept_trait(&mut f, base_name, &types);

    define_visitor(&mut f, base_name, &types);
}

fn define_accept_trait(file: &mut File, base_name: &str, types: &Vec<String>) {
    file.write(
        "pub trait Accept<R> {\n    fn accept(&self, visitor: &dyn Visitor<R>) -> R;\n}\n"
            .as_bytes(),
    )
    .unwrap();

    file.write(format!("impl<R> Accept<R> for {} {{\n", base_name).as_bytes())
        .unwrap();
    file.write("    fn accept(&self, visitor: &dyn Visitor<R>) -> R {\n".as_bytes())
        .unwrap();
    file.write("        match self {\n".as_bytes()).unwrap();

    for typ in types {
        let mut s = typ.split(":");
        let class_name = s.next().unwrap().trim();
        file.write(
            format!(
                "{}::{}(a) => visitor.visit_{}_{}(a),\n",
                base_name,
                class_name,
                class_name.to_lowercase(),
                base_name.to_lowercase(),
            )
            .as_bytes(),
        )
        .unwrap();
    }

    file.write("        }\n    }\n}\n".as_bytes()).unwrap();
}

fn define_enum_type(file: &mut File, base_name: &str, types: &Vec<String>) {
    file.write(format!("pub enum {} {{ \n", base_name).as_bytes())
        .unwrap();
    types.iter().for_each(|ty| {
        let mut s = ty.split(":");
        let class_name = s.next().unwrap().trim();
        file.write(format!("    {}({}),\n", class_name, class_name).as_bytes())
            .unwrap();
    });
    file.write("} \n".as_bytes()).unwrap();
}

fn define_struct_types(file: &mut File, base_name: &str, types: &Vec<String>) {
    for typ in types {
        let mut s = typ.split(":");
        let class_name = s.next().unwrap().trim();
        let fields = s.next().unwrap().trim();

        file.write(format!("pub struct {} {{\n", class_name).as_bytes())
            .unwrap();

        fields.split(",").for_each(|field| {
            let mut s = field.trim().split(" ");
            let ty = s.next().unwrap().trim();
            let name = s.next().unwrap().trim();

            let ty = ty_map::map_to_rs_type(ty, base_name);

            file.write(format!("    pub {}: {},\n", name, ty).as_bytes())
                .unwrap();
        });
        file.write("}\n".as_bytes()).unwrap();
    }
}

fn define_visitor<'a>(file: &mut File, base_name: &str, types: &Vec<String>) {
    file.write(format!("pub trait Visitor<R> {{\n").as_bytes())
        .unwrap();
    for ty in types {
        let mut s = ty.split(":");
        let class = s.next().unwrap().trim();

        file.write(
            format!(
                "    fn visit_{}_{}(&self, {}: &{}) -> R;\n",
                class.to_lowercase(),
                base_name.to_lowercase(),
                class.to_lowercase(),
                class,
            )
            .as_bytes(),
        )
        .unwrap();
    }
    file.write("}".as_bytes()).unwrap();
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

fn main() {
    let out_dir = generate_ast();

    let v = vec![
        "Binary   : Expr left, Token operator, Expr right".to_string(),
        "Grouping : Expr expression".to_string(),
        "Literal  : Object value".to_string(),
        "Unary    : Token operator, Expr right".to_string(),
    ];

    define_ast(&out_dir, "Expr", v);
}
