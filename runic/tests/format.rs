use runic::Runic;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod utils;

mod format {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[wasm_bindgen_test]
    fn _0() {
        assert_eq!(Runic::translate(duration! {::0} as f64), "0h 0m 0s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _1h() {
        assert_eq!(Runic::translate(duration! {01:00} as f64), "1h 0m 0s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _2h() {
        assert_eq!(Runic::translate(duration! {02:00} as f64), "2h 0m 0s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _1m() {
        assert_eq!(Runic::translate(duration! {00:01} as f64), "0h 1m 0s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _2m() {
        assert_eq!(Runic::translate(duration! {00:02} as f64), "0h 2m 0s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _1s() {
        assert_eq!(Runic::translate(duration! {::1} as f64), "0h 0m 1s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _2s() {
        assert_eq!(Runic::translate(duration! {::2} as f64), "0h 0m 2s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _1h_1m_1s() {
        assert_eq!(Runic::translate(duration! {01:01-01} as f64), "1h 1m 1s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _3600s() {
        assert_eq!(Runic::translate(duration! {::3600} as f64), "1h 0m 0s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _3600s_90m() {
        assert_eq!(Runic::translate(duration! {00:90-3600} as f64), "2h 30m 0s");
    }

    #[test]
    #[wasm_bindgen_test]
    fn _120m_125s() {
        assert_eq!(Runic::translate(duration! {02:02-5} as f64), "2h 2m 5s");
    }
}
