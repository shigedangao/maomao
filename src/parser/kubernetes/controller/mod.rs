use std::collections::HashMap;
use toml::Value;
use crate::kubernetes::controller::K8SController;
use crate::helper::err::LibError;
use super::container::Container;
use super::KubernetesResources;

// Error Constant
const MISSING_TEMPLATE: &str = "[spec] field is missing";
const MISSING_TEMPLATE_MSG: &str = "Please check that [spec] exist";

const MISSING_CONTAINERS: &str = "[spec.containers] is missing";
const MISSING_CONTAINER_MSG: &str = "Make sure that [spec.containers] is under the [spec] field";

const MALFORMATTED_CONTAINERS: &str = "Make sure that [spec.containers] is a TOML table field";
const MALFORMATTED_CONTAINERS_MSG: &str = "[spec.containers] must be a TOML table field";

const ERROR_CREATE_CONTAINER: &str = "Error while creating containers";


#[derive(Default, Debug)]
pub struct Controller {
    kind: K8SController,
    containers: Vec<Container>,
    metadata: HashMap<String, String>
}

impl KubernetesResources for Controller {
    fn new() -> Self {
        Controller::default()
    }

    fn set_metadata(mut self, labels: Option<HashMap<String, String>>) -> Self {
        if labels.is_none() {
            return self;
        }

        self.metadata = labels.unwrap();
        self
    }
}

impl Controller {
    /// Set Containers
    ///
    /// # Description
    /// Set containers to the controller object
    ///
    /// # Arguments
    /// * `content` - Option<Value>
    fn set_containers(mut self, content: Option<Value>) -> Result<Self, LibError> {
        let templates = content.ok_or(LibError {
            kind: MISSING_TEMPLATE.to_owned(),
            message: MISSING_TEMPLATE_MSG.to_owned()
        })?;

        let spec_field = templates.get("spec").ok_or_else(|| LibError {
            kind: MISSING_TEMPLATE.to_owned(),
            message: MISSING_TEMPLATE_MSG.to_owned()
        })?;

        let containers = spec_field.get("containers").ok_or_else(|| LibError {
            kind: MISSING_CONTAINERS.to_owned(),
            message: MISSING_CONTAINER_MSG.to_owned()
        })?;

        let containers_list = containers.as_table().ok_or_else(|| LibError {
            kind: MALFORMATTED_CONTAINERS.to_owned(),
            message: MALFORMATTED_CONTAINERS_MSG.to_owned()
        })?;

        let mut containers = Vec::new();
        for (_, v) in containers_list.iter() {
            let res = Container::new(v)?;
            let c = res.set_env(v).set_probes(v);

            containers.push(c);
        }

        self.containers = containers;

        Ok(self)
    }
}

#[cfg(test)]
mod test_controller {
    use toml::Value;
    use std::collections::HashMap;
    use crate::{kubernetes::controller::K8SController, parser::kubernetes::KubernetesResources};
    use super::Controller;

    #[test]
    fn test_set_metadata() {
        let mut map = HashMap::new();
        map.insert("foo".to_owned(), "bar".to_owned());

        let controller = Controller::new().set_metadata(Some(map));

        assert_eq!(controller.metadata.get("foo").unwrap(), "bar");
    }

    #[test]
    fn test_set_containers() {
        let content = "
            [spec]
            replicas = 1

                [spec.containers]

                    [spec.containers.node]
                    name = 'node'
                    image = 'node:$tag'
                    ports = [
                        { name = 'http', value = '$port' }
                    ]

                    [spec.containers.node.env]
                        map = [
                            # will make reference to a set of econfigmap / secrets
                            'configmap::misc',
                            'configmap::api',
                            'secrets::foo'
                        ]
                        from = [
                            # will make a reference to a set of secrets variables
                            { kind = 'secret', name = 'google-api-key', from = 'google::main.key' }
                        ]
                        raw = [
                            # raw kubernetes env value
                            { name = 'greeting', value = 'bar' }
                        ]

                    [spec.containers.node.probes]

                        [spec.containers.node.probes.liveness]
                            kind = 'http'
                            path = 'foo'
                            port = 8080                
                            http_headers = [
                                { name = 'baz', value = 'wow' },
                                { name = 'yo', value = 'sabai' }
                            ]            
        ";

        let spec = content.parse::<Value>().unwrap();
        let controller = Controller::new().set_containers(Some(spec)).unwrap();

        assert!(controller.containers.len() != 0);
        assert_eq!(controller.kind, K8SController::Deployment);

        let container = controller.containers.get(0).unwrap();
        assert_eq!(container.name, "node");
        assert_eq!(container.image, "node:$tag");
        
        // retrieve env
        let env = container.env.as_ref().unwrap();
        
        let map = env.map.as_ref().unwrap();
        assert_eq!(map.get(0).unwrap(), "configmap::misc");

        // retrieve raw
        let raw = env.raw.as_ref().unwrap();
        assert_eq!(raw.get(0).unwrap().name, "greeting");
        assert_eq!(raw.get(0).unwrap().value, "bar");

        // retrieve from env
        let from = env.from.as_ref().unwrap();
        assert_eq!(from.get(0).unwrap().name, "google-api-key");
        assert_eq!(from.get(0).unwrap().from, "google::main.key");

        let probes = container.probes.as_ref().unwrap();
        assert!(probes.liveness.is_some());
        assert!(probes.readiness.is_none());

        let liveness = probes.liveness.as_ref().unwrap();
        assert!(liveness.http_get.is_some());
        assert_eq!(liveness.initial_delay_seconds, 0);
        assert_eq!(liveness.period_seconds, 0);
        
        let http_get = liveness.http_get.as_ref().unwrap();
        assert_eq!(http_get.path, "foo");
        assert_eq!(http_get.port, 8080);
    }
}