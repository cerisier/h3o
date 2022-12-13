use super::h3api;
use float_eq::assert_float_eq;
use h3o::LatLng;

const ICECREAM: (f64, f64) = (48.854501508844095_f64, 2.3729695423293613_f64);
const BOCAMEXA: (f64, f64) = (48.854091837885264_f64, 2.3708719883564124_f64);
const GOOGLE: (f64, f64) = (37.422747247604335_f64, -122.08389658095136_f64);

macro_rules! test {
    ($name:ident, $src:ident, $dst:ident) => {
        #[test]
        fn $name() {
            let src = LatLng::new($src.0.to_radians(), $src.1.to_radians())
                .expect("valid location");
            let dst = LatLng::new($dst.0.to_radians(), $dst.1.to_radians())
                .expect("valid location");
            let result = src.distance_km(dst);
            let reference = h3api::great_circle_distance_km(&src, &dst);

            assert_float_eq!(result, reference, r2nd <= f64::EPSILON);
        }
    };
}

test!(zero, ICECREAM, ICECREAM);
test!(close, ICECREAM, BOCAMEXA);
test!(far, ICECREAM, GOOGLE);
