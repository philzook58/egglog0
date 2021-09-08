use std::sync::atomic::{AtomicUsize, Ordering};

static GENSYM_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn fresh() -> usize {
    GENSYM_COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn gensym(prefix: &str) -> String {
    format!("#{}#{}", prefix, fresh())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works2() {
        assert_eq!(fresh(), 0);
        assert_eq!(fresh(), 1);
        assert_eq!(gensym("fred"), "#fred#2");
    }
}
