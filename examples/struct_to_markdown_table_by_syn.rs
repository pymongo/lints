fn main() {
    const CODE: &str = r#"
    // clap or structopt
    struct CliArg {
        /// doc of input_file
        input_file: String,
        /// doc of output_file line 1
        /// doc of output_file line 2
        output_file: String
    }
    "#;

    // markdown header
    println!("| arg_name | type | default_value | description |");
    println!("| --- | --- | --- | --- |");

    let struct_ = syn::parse_str::<syn::ItemStruct>(CODE).unwrap();
    for field in struct_.fields {
        // assert is named struct
        let field_name = field.ident.unwrap().to_string();
        let mut doc = String::new();
        let field_type = if let syn::Type::Path(type_path) = field.ty {
            type_path.path.get_ident().unwrap().to_string()
        } else {
            unreachable!()
        };
        for attr in field.attrs {
            if attr.path.get_ident().unwrap() == "doc" {
                for token in attr.tokens {
                    if let quote::__private::TokenTree::Literal(lit) = token {
                        doc.push_str(&lit.to_string());
                        break;
                    }
                }
            }
        }
        println!(
            "| {arg_name} | {type_} | {default_value} | {description}",
            arg_name = field_name,
            type_ = field_type,
            default_value = "",
            description = doc
        );
    }
}
