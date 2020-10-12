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