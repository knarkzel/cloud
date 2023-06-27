use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, FnArg, ItemFn, PatType, Pat, PatIdent};

#[proc_macro_attribute]
pub fn cloud(_: TokenStream, input: TokenStream) -> TokenStream {
    // Parse ast from token stream
    let ast = syn::parse::<ItemFn>(input).expect("Place this attribute above a function");

    // Signature and output type
    let name = ast.sig.ident;
    let output = ast.sig.output;
    let stub_name = format_ident!("{name}_stub");

    // Stub
    let body = ast.block.stmts;
    let stub = match ast.sig.inputs.first() {
        Some(FnArg::Typed(PatType { pat, ty, .. })) => match &**pat {
            Pat::Ident(PatIdent { mutability, ident, ..}) => quote! {
                let #ident = ::cloud_lib::rmp_serde::from_slice::<#ty>(__input_bytes).unwrap();

                // Actual code
                fn #stub_name(#mutability #ident: #ty) #output {
                    #(#body);*
                }
                let output = #stub_name(#ident);

                // Serialize output
                let __output_bytes = ::cloud_lib::rmp_serde::to_vec_named(&output).unwrap().leak();
                ::std::boxed::Box::new(::cloud_lib::CloudSlice {
                    len: __output_bytes.len(),
                    ptr: __output_bytes.as_mut_ptr(),
                })
            },
            _ => Error::new(name.span(), "expected ident").to_compile_error(),
        },
        _ => Error::new(name.span(), "expected one argument").to_compile_error(),
    };

    quote! {
        #[no_mangle]
        fn #name(ptr: usize) -> ::std::boxed::Box<::cloud_lib::CloudSlice> {
            // Deserialize input
            let __input_bytes = unsafe {
                let len = *(ptr as *const usize);
                let data = (ptr + 4) as *mut u8;
                ::std::slice::from_raw_parts(data, len)
            };

            #stub
        }
    }
    .into()
}
