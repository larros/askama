extern crate iron;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(WriteBody)]
pub fn derive_writebody(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    let generics = &ast.generics;
    let name = &ast.ident;
    let where_clause = &ast.generics.where_clause;
    let tokens = quote! {
        impl #generics ::iron::response::WriteBody for #name #generics #where_clause {
            fn write_body(&mut self, res: &mut ::std::io::Write) -> ::std::io::Result<()> {
                res.write_all(self.render().unwrap().as_bytes())
            }
        }
    };
    eprintln!("{}", tokens.as_str());
    tokens.as_str().parse().unwrap()
}
