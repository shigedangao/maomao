#[derive(Debug, PartialEq)]
pub enum K8SController {
    Deployment,
    ReplicaSet,
    StatefulSet,
    DaemonSets,
    Jobs,
    CronJob,
    ReplicationController
}

impl Default for K8SController {
    fn default() -> Self {
        return K8SController::Deployment
    }
}