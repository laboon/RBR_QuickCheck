extern crate quickcheck;

// Floating-point absolute value function

fn fabs(x: f32) -> f32 {
    if x >= 0.0 { x } else { -x }
}

#[cfg(test)]

mod tests {

    // Get access to the fabs function
    
    use super::fabs;

    // QuickCheck imports
    
    use quickcheck::quickcheck;
    use quickcheck::TestResult;
    
    // A classic unit test, for comparison.
    // Just checks that the absolute value of -1 is 1.
    // Note that we are only checking one specific value
    // and one specific output.

    #[test]
    fn test_fabs_classic() {
        assert_eq!(1.0, fabs(-1.0));
    }

    // A property-based test.  Note that there are no
    // explicit assertions - they are "built-in" to the
    // quickcheck function.  However, they can be
    // annotated as #[test] just like a normal unit test.

    // Absolute values should never be negative

    #[test]
    fn test_fabs_never_negative() {
        // Define a property or properties here.
        // For Rust's version of QuickCheck, I have found
        // that it's better to define one property per test.
        // This makes reading the output much easier.

        // These properties can also be defined elsewhere
        // if you would like to re-use them.  However,
        // I find that keeping them in the test block
        // is easiest for understanding.
        
        fn prop_no_neg(x: f32) -> bool {
            fabs(x) >= 0.0
        }

        // Now run with many (100 by default) pseudorandom f32
        // values and check that the property holds true (i.e.,
        // the function prop_no_neg returns true) for all of them.
        // If not, the test will fail.

        // Note that you need to have the "as" section
        quickcheck(prop_no_neg as fn(f32) -> bool);
    }

    // Absolute values should always be same distance from 0
    // as original

    #[test]
    fn test_fabs_same_distance() {
        
        fn prop_fabs_same_distance(x: f32) -> bool {
            let ax = fabs(x);
            ax == x || ax == -x
        }

        quickcheck(prop_fabs_same_distance as fn(f32) -> bool);
    }

    // Absolute values should always return same value

    #[test]
    fn test_fabs_always_same() {

        fn prop_fabs_always_same(x: f32) -> bool {
            fabs(x) == fabs(x)
        }
        
        quickcheck(prop_fabs_always_same as fn(f32) -> bool);
    }

    // Absolute values should be idempotent

    #[test]
    fn test_fabs_idempotent() {
        fn prop_fabs_idempotent(x: f32) -> bool {
            fabs(x) == fabs(fabs(x))
        }
        quickcheck(prop_fabs_idempotent as fn(f32) -> bool);
    }

    // If a number is not negative, the absolute value of x should
    // be equal to x.

    // This is more complicated!  We need to throw away any
    // randomly generated pair where y is equal to 0.0.
    // First, note that we are generating TWO different
    // arguments to pass in (this necessitates changing the "as"
    // section to match the function signature).  

    // Secondly, we need to say that some tests are invalid
    // (any where y == 0).  We can return a TestResult instead of
    // a regular old boolean value, but this will complicate things
    // a bit.

    // Finally, note that values are generated and then discarded
    // later.  If too many values are generated which are discarded,
    // quickcheck will eventually give up (specifically, if it can't
    // find 100 examples after 10,000 tries, it will stop trying).


    #[test]
    fn test_fabs_nonnegative_equal() {

        // Note that we are returning a TestResult here
        fn prop_nonnegative_equal(x: f32) -> TestResult {
            if x < 0.0 {
                // Pseudorandom value was negative, ignore
                // this test case and try again
                TestResult::discard()
            } else {
                // Pseudorandom value was nonnegative, we
                // can test it

                // Note that we are returning a TestResult!
                TestResult::from_bool(fabs(x) == x)
            }
        }
        
        // we are returning a TestResult here as well
        quickcheck(prop_nonnegative_equal as fn(f32) -> TestResult);
    }

    // If a number is negative, the absolute value of x should
    // be greater than x.

    #[test]
    fn test_fabs_negative_gt() {

        // Note that we are returning a TestResult here
        fn prop_negative_gt(x: f32) -> TestResult {
            if x >= 0.0 {
                // Pseudorandom value was nonnegative, ignore
                // this test case and try again
                TestResult::discard()
            } else {
                // Pseudorandom value was negative, we
                // can test it

                // Note that we are returning a TestResult!
                TestResult::from_bool(fabs(x) > x)
            }
        }
        
        // we are returning a TestResult here as well
        quickcheck(prop_negative_gt as fn(f32) -> TestResult);
    }

    
}
