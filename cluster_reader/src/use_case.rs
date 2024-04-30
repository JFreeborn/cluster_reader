pub mod use_case {
    
    use std::io::{Error, BufReader, BufRead, ErrorKind};
    use std::process::{Command, Stdio};
    use serde::{Deserialize, Serialize};
    use regex::Regex;

    pub async fn handle() -> Result<ClusterValues, Error> {
        let z = get_node_list().await?;
        let x = get_node_description(&z).await?;
        let c = process_node_description(&x).await?;

        Ok(c)
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    pub struct NodeList {
        node_name: Vec<String>
    }

    pub async fn get_node_list() -> Result<NodeList, Error> {

        let get_node_list_command = Command::new("kubectl")
            .arg("get")
            .arg("nodes")
            .arg("--output=name")
            .stdout(Stdio::piped())
            .spawn()?;

            let mut node_list = NodeList {
                node_name: Vec::new()
            };

            if let Some(stdout) = get_node_list_command.stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    let line = line?;

                    node_list.node_name.push(line);
                }
            } else {
                return Err(Error::new(ErrorKind::Other, "Failed to capture stdout in get_node_list"));
            }
        
        node_list.node_name.sort();

        Ok(node_list)
    }

/**********************************************************************************************************/

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    pub struct NodesAndDescriptionList {
        node_list: Vec<NodeAndDescription>,
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    struct NodeAndDescription {
        node_name: String,
        node_description: String
    }

    pub async fn get_node_description(node_list: &NodeList) -> Result<NodesAndDescriptionList, Error> {

        let mut node_and_description_list = NodesAndDescriptionList {
            node_list: Vec::new()
        };

        for node in &node_list.node_name{
            let get_node_description_command = Command::new("kubectl")
                .arg("describe")
                .arg(&node)
                .stdout(Stdio::piped())
                .spawn()?;

                let mut node_and_description = NodeAndDescription {
                    node_name: String::from(&node.to_string()), 
                    node_description: String::new()
                };

                if let Some(stdout) = get_node_description_command.stdout {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        let line = line?;
    
                        node_and_description.node_description.push_str(&line);
                        node_and_description.node_description.push('\n');
                        
                    }
                } else {
                    return Err(Error::new(ErrorKind::Other, "Failed to capture stdout from node description"));
                }

                node_and_description_list.node_list.push(node_and_description);
        }

        Ok(node_and_description_list)
    }

/**********************************************************************************************************/

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    struct Labels {
        labels: Vec<String>
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    struct Annotations {
        annotations: Vec<String>
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    struct Capacity {
        cpu: i32,
        ephemeral_storage_ki: i32,
        hugepages_2mi: i32,
        memory_ki: i32,
        pods: i32,
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    struct Allocatable {
        cpu: i32,
        ephemeral_storage_bytes: i64,
        hugepages_2mi: i32,
        memory_ki: i32,
        pods: i32,
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    struct FinalNodeValues {
        name: String,
        roles: String,
        labels: Labels,
        annotations: Annotations,
        created_date: String,
        capacity: Capacity,
        allocatable: Allocatable
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    pub struct ClusterValues {
        nodes: Vec<FinalNodeValues>
    }

    pub async fn process_node_description(node_and_description_list: &NodesAndDescriptionList) -> Result<ClusterValues, Error> {

        const REGEX_PATTERN: &str = r"(^Name\:\s+)(.*)\n(Roles\:\s+)(.*)\n(Labels\:\s+)((.*\n)+)(Annotations\:\s+)((.*\n)+)(CreationTimestamp\:\s+)(.*)((.*\n)+)(Capacity\:)((.*\n)+)(Allocatable\:)((.*\n)+)(System Info\:\n)";

        let regex_pattern = Regex::new(REGEX_PATTERN).unwrap();

        let mut clutser_values = ClusterValues {
            nodes: Vec::new()
        };

        for node in &node_and_description_list.node_list{
            if let Some(captures) = regex_pattern.captures(&node.node_description) {
                if let (
                    Some(node_name_value),
                    Some(node_role_value),
                    Some(node_labels_value),
                    Some(node_annotations_value),
                    Some(node_created_value),
                    Some(node_capacity_value),
                    Some(node_allocatable_value)
                ) = (
                    captures.get(2),
                    captures.get(4),
                    captures.get(6),
                    captures.get(9),
                    captures.get(12),
                    captures.get(16),
                    captures.get(19)
                ) {
                    let get_labels = process_labels_string_into_array(&node_labels_value.as_str())?;
        
                    let get_annotations = process_annotations_string_into_vector(&node_annotations_value.as_str())?;
                
                    let get_capacity = process_capacity_into_value(&node_capacity_value.as_str())?;
        
                    let get_allocatable = process_allocatable_into_value(&node_allocatable_value.as_str())?;
        
                    let final_values = FinalNodeValues {
                        name: String::from(node_name_value.as_str().trim()),
                        roles: String::from(node_role_value.as_str().trim()),
                        created_date: String::from(node_created_value.as_str().trim()),
                        annotations: get_annotations,
                        labels: get_labels,
                        capacity: get_capacity,
                        allocatable: get_allocatable
                    };
        
                    clutser_values.nodes.push(final_values);
                }
            }
        }

        Ok(clutser_values)
    }

    fn process_labels_string_into_array(input: &str) -> Result<Labels, Error>{

        let mut labels_vector = Labels {
            labels: Vec::new(),
        };
    
        for line in input.lines() {
            labels_vector.labels.push(String::from(line.trim())); 
        }
    
        Ok(labels_vector)    
    }

    fn process_annotations_string_into_vector(input: &str) -> Result<Annotations, Error> {
        let mut annotations_vector = Annotations {
            annotations: Vec::new()
        };
    
        for line in input.lines() {
            
            annotations_vector.annotations.push(String::from(line.trim()));
        }
    
        Ok(annotations_vector)
    }

    fn process_capacity_into_value(input: &str) -> Result<Capacity, Error> {

        // Initialize the struct with default values
        let mut resource = Capacity {
           cpu: 0,
           ephemeral_storage_ki: 0,
           hugepages_2mi: 0,
           memory_ki: 0,
           pods: 0,
       };
   
       // Splitting the multiline string into separate lines
       for line in input.lines() {
           // Split each line into key and value
           let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
           if parts.len() == 2 {
               let key = parts[0];
               let value = parts[1].trim().trim_end_matches("Ki").parse::<i32>().unwrap_or(0); // Parsing the value as integer
               match key {
                   "cpu" => resource.cpu = value,
                   "ephemeral-storage" => resource.ephemeral_storage_ki = value,
                   "hugepages-2Mi" => resource.hugepages_2mi = value,
                   "memory" => resource.memory_ki = value,
                   "pods" => resource.pods = value,
                   _ => {}
               }
           }
       }
   
       Ok(resource)
   }

    fn process_allocatable_into_value(input: &str) -> Result<Allocatable, Error>{

        // Initialize the struct with default values
        let mut resource = Allocatable {
            cpu: 0,
            ephemeral_storage_bytes: 0,
            hugepages_2mi: 0,
            memory_ki: 0,
            pods: 0,
        };

        // Splitting the multiline string into separate lines
        for line in input.lines() {
            // Split each line into key and value
            let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value = parts[1].trim().trim_end_matches("Ki").parse::<i32>().unwrap_or(0); // Parsing the value as integer
                match key {
                    "cpu" => resource.cpu = value,
                    "ephemeral-storage" => resource.ephemeral_storage_bytes = parts[1].trim().parse::<i64>().unwrap_or(0),
                    "hugepages-2Mi" => resource.hugepages_2mi = value,
                    "memory" => resource.memory_ki = value,
                    "pods" => resource.pods = value,
                    _ => {}
                }
            }
        }

        Ok(resource)
    }


/**********************************************************************************************************/
/**********************************************************************************************************/
/**********************************************************************************************************/
/**********************************************************************************************************/
/**********************************************************************************************************/

    pub async fn get_namespace_details_handler() -> Result<AllNamespaceDetails, Error> {
        
        let z = get_namespaces().await?;
        
        let x = get_deployments_and_details_by_namespace(&z).await?;

        Ok(x)
    }


    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    pub struct Namespaces {
        namesapces: Vec<String>
    }

    pub async fn get_namespaces() -> Result<Namespaces, Error> {

        let mut namespaces = Namespaces {
            namesapces: Vec::new()
        };

        let get_namesapces_command = Command::new("kubectl")
            .arg("get")
            .arg("namespaces")
            .arg("--output=name")
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(stdout) = get_namesapces_command.stdout {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                let line = line?;

                let parts: Vec<&str> = line.split('/').collect();
                
                if let Some(item_two) = parts.get(1) {
                    namespaces.namesapces.push(String::from(item_two.to_string()));
                } else {
                    println!("The string doesn't have a second part.");
                }
            }
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to capture stdout from get namespace command"));
        }

        namespaces.namesapces.sort();

        Ok(namespaces)
    }


/**********************************************************************************************************/

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    pub struct DeploymentDetails {
        deployment: String, 
        details: String
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    pub struct NamespaceDetails {
        namespace: String,
        deployment_details: Vec<DeploymentDetails>
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[derive(Serialize)] 
    pub struct AllNamespaceDetails {
        all_namespace_details: Vec<NamespaceDetails>
    }

    pub async fn get_deployments_and_details_by_namespace(namespaces_list: &Namespaces) -> Result<AllNamespaceDetails, Error> {
        
        let mut all_namespace_details = AllNamespaceDetails {
            all_namespace_details: Vec::new()
        };

        for namespace in &namespaces_list.namesapces{
            
            let mut namespace_details = NamespaceDetails {
                namespace: String::from(namespace),
                deployment_details: Vec::new(),
            };

            // Get kubectl get deployments -n default --output=name
            let get_deployments_by_namespace_command = Command::new("kubectl")
                .arg("get")
                .arg("deployments")
                .arg("-n")
                .arg(&namespace)
                .arg("--output=name")
                .stdout(Stdio::piped())
                .spawn()?;

            if let Some(stdout) = get_deployments_by_namespace_command.stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    let line = line?;
    
                    let mut depolyment_details = DeploymentDetails {
                        deployment: String::new(),
                        details: String::new(),
                    };

                    let parts: Vec<&str> = line.split('/').collect();
                
                    let mut deployment_name = String::new();

                    if let Some(item_two) = parts.get(1) {
                        depolyment_details.deployment.push_str(&item_two);
                        deployment_name = item_two.to_string(); 
                    } else {
                        println!("The string doesn't have a second part.");
                    }

                    // kubectl get deployment coredns -n kube-system -o yaml
                    let get_deployment_details_command = Command::new("kubectl")
                        .arg("get")
                        .arg("deployment")
                        .arg(&deployment_name)
                        .arg("-n")
                        .arg(&namespace)
                        .arg("-o")
                        .arg("yaml")
                        .stdout(Stdio::piped())
                        .spawn()?;

                    if let Some(stdout) = get_deployment_details_command.stdout {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            let line = line?;
        
                            depolyment_details.details.push_str(&line);
                            depolyment_details.details.push('\n');
                            
                        }
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Failed to capture stdout from node description"));
                    }

                    println!("{}", depolyment_details.details);

                    // Then add to the NamespaceDetails
                    namespace_details.deployment_details.push(depolyment_details);
                }
            } else {
                return Err(Error::new(ErrorKind::Other, "Failed to capture stdout from get namespace command"));
            }
            all_namespace_details.all_namespace_details.push(namespace_details);
        }
        
        Ok(all_namespace_details)
    }
}