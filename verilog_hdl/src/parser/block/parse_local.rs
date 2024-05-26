use crate::generate_main::Serializer;

impl Serializer {
    pub(crate) fn parse_local(&mut self, local: &syn::Local) {
        let syn::Local {
            attrs: _,
            let_token: _,
            pat: _,
            init, // e.g. num = 1
            semi_token: _,
        } = local;

        if let Some(init) = init {
            // - let x: u64 = s.parse()?
            // - let Ok(x) = r else { return }
            let syn::LocalInit {
                eq_token: _,
                expr: _,
                diverge: _,
            } = init;
        };
    }
}
