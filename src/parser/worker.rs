use super::template::{
    parse_template,
    Template,
    TemplateKind
};
use crate::helper::err::LibError;
use super::kubernetes::get_kubernetes_kind_object;

/// Prepare
///
/// # Description
/// Iterate throught each templates
///
/// # Arguments
/// * `content` - Vec<String>
pub fn prepare(content: Vec<String>) -> Result<(), LibError> {
    for tmpl in content {
        let ptmpl = parse_template(tmpl);
        match ptmpl {
            Ok(res) => prepare_template(res),
            Err(err) => return Err(err)
        };
    }

    Ok(())
}

/// Prepare Template
///
/// # Description
/// Prepare template based on it's type
fn prepare_template(tm: Template) {
    let res = match tm.kind {
        TemplateKind::K8SObject(k8s_type) => get_kubernetes_kind_object(k8s_type, tm.content),
        _ => Ok(())
    };

    //println!("{:?}", res);
}


#[cfg(test)]
mod worker_test {
    use crate::io;

    #[test]
    fn prepare_templates_example() {
        let content = io::read::templates::read_templates("./examples/node").unwrap();
        super::prepare(content);
    }
}