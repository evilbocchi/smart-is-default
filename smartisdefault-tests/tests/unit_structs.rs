use smart_is_default::SmartIsDefault;

#[derive(Default, SmartIsDefault, PartialEq, Debug)]
struct Marker;

#[test]
fn unit_struct_is_default_reports_true() {
    assert!(Marker.is_default());
}

#[derive(Default, SmartIsDefault, PartialEq, Debug)]
#[smart_is_default(no_is_default)]
struct CustomMarker;

impl CustomMarker {
    fn is_default(&self) -> bool {
        false
    }
}

#[test]
fn unit_struct_respects_no_is_default() {
    assert!(!CustomMarker.is_default());
}
