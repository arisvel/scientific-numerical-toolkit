#[cfg(test)]
mod bisection_tests {
    use snt::optimize::root_finding::Bisection;

    #[test]
    fn test_quadratic() {
        let root1 = Bisection::initialize(|x| x * x - 4.0, -3.0, -1.0)
            .tol(1e-5)
            .run();

        let root2 = Bisection::initialize(|x| x * x - 4.0, 1.0, 3.0)
            .tol(1e-5)
            .run();

        match root1 {
            Ok(root1) => {
                assert!((root1 + 2.0).abs() < 1e-5)
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
        match root2 {
            Ok(root2) => {
                assert!((root2 - 2.0).abs() < 1e-5)
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        };
    }
    #[test]
    fn test_quadratic_high_precision() {
        let root1 = Bisection::initialize(|x| x * x - 4.0, -3.0, -1.0)
            .tol(1e-10)
            .iter(10000)
            .run();
        let root2 = Bisection::initialize(|x| x * x - 4.0, 1.0, 3.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root1 {
            Ok(root1) => {
                assert!((root1 + 2.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
        match root2 {
            Ok(root2) => {
                assert!((root2 - 2.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        };
    }
    #[test]
    fn test_sine() {
        let root1 = Bisection::initialize(|x| x.sin(), -1.0, 1.0)
            .tol(1e-5)
            .run();
        let root2 = Bisection::initialize(|x| x.sin(), 2.0, 4.0).tol(1e-5).run();

        match root1 {
            Ok(root1) => {
                assert!(root1.abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
        match root2 {
            Ok(root2) => {
                assert!((root2 - std::f64::consts::PI).abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        };
    }
    #[test]
    fn test_sine_high_precision() {
        let root1 = Bisection::initialize(|x| x.sin(), -1.0, 1.0)
            .tol(1e-10)
            .iter(10000)
            .run();
        let root2 = Bisection::initialize(|x| x.sin(), 2.0, 4.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root1 {
            Ok(root1) => {
                assert!(root1.abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
        match root2 {
            Ok(root2) => {
                assert!((root2 - std::f64::consts::PI).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        };
    }
    #[test]
    fn test_exponential() {
        let root = Bisection::initialize(|x| x.exp() - 2.0, 0.0, 1.0)
            .tol(1e-5)
            .run();
        match root {
            Ok(root) => {
                assert!((root - 2.0f64.ln()).abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test_exponential_high_precision() {
        let root = Bisection::initialize(|x| x.exp() - 2.0, 0.0, 1.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root {
            Ok(root) => {
                assert!((root - f64::ln(2.0)).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test_complex_high_precision() {
        let root = Bisection::initialize(|x| x.powi(3) - 2.0 * x.powi(2) + x.sin() - 1.0, 0.0, 3.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root {
            Ok(root) => {
                // Validate the root within the tolerance
                let f_root = root.powi(3) - 2.0 * root.powi(2) + root.sin() - 1.0;
                assert!(f_root.abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }

    #[test]
    fn test_super_complex_high_precision() {
        let root = Bisection::initialize(|x| (-x).exp() + x.powi(2) - x.cos() - 1.0, -2.0, 0.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root {
            Ok(root) => {
                // Validate the root within the tolerance
                let f_root = (-root).exp() + root.powi(2) - root.cos() - 1.0;
                assert!(f_root.abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }

    #[test]
    fn test_linear() {
        let root = Bisection::initialize(|x| x, -1.0, 1.0).tol(1e-5).run();

        match root {
            Ok(root) => {
                assert!(root.abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test_log_poly_high_precision() {
        let root = Bisection::initialize(|x| (x + 1.0).ln() - x.powi(2) + 2.0 * x, -0.5, 2.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root {
            Ok(root) => {
                // Validate the root within the tolerance
                let f_root = (root + 1.0).ln() - root.powi(2) + 2.0 * root;
                assert!(f_root.abs() < 1e-8);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
}
