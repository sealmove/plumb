use hcl::from_str;
use plumb::Pipeline;

#[test]
fn parse_pipeline_from_hcl() {
    let hcl_str = r#"
        node "hello" {
            command = "echo Hello"
        }
    "#;

    let pipeline: Pipeline = from_str(hcl_str).expect("Failed to parse HCL");

    assert_eq!(pipeline.node["hello"].command, "echo Hello");
}
