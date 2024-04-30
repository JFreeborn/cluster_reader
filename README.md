# cluster_reader
This is the start to the cluster reader service that will serve up cluster info in JSON via API endpoints.





### Regex Patterns to use.

* Parse apart the deployment string result: 

    `(^apiVersion\:\s)(.*)\n(kind\:\s)(Deployment)\n(metadata\:)((.*\n)+)(spec\:)((.*\n)+)(status\:)((.*\n)+.*)`

    This one ends with an extra .* in order to get the last line, not sure if we will need it but this way we have it.

