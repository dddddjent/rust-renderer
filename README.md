# The Renderer

<!--toc:start-->
- [The Renderer](#the-renderer)
  - [Configuration](#configuration)
    - [What should the Configuration do](#what-should-the-configuration-do)
    - [Modify the Configuration](#modify-the-configuration)
      - [The config.json](#the-configjson)
  - [Data Process](#data-process)
    - [What should a `DataProcessor` do](#what-should-a-dataprocessor-do)
    - [Add New `DataProcessor`](#add-new-dataprocessor)
    - [DataProcessors](#dataprocessors)
      - [Direct](#direct)
  - [World](#world)
    - [Structure](#structure)
    - [Methods](#methods)
    - [Objects](#objects)
      - [Why `enum` not `dyn trait`](#why-enum-not-dyn-trait)
    - [Mesh](#mesh)
      - [Add New Mesh Type](#add-new-mesh-type)
    - [Material](#material)
      - [Add new Material Type](#add-new-material-type)
<!--toc:end-->

## Configuration

### What should the Configuration do

- Get the configuration of the project
- Use the configuration information to get the `World`, `Renderer`, `Consumer`
  - It's divided into three stages: `data_process`, `renderer`, `consumer`
  - The `world` is produced inside the `data_process` stage, but it's passed out directly, so `data_processor` and `Configuration` should not contain any state or memory
  - Consumer should be able to use the configuration to change the `World`, `Renderer` dynamically. (Pass it to the consumer?)
  - It should only use the trait methods, so you should never need to change the implementations of these three stages

### Modify the Configuration

I think in most cases the configuration is deserialized directly from
the [project_path]/config.json. So:

- Change the structure of `Configuration` struct to accept new config.json

#### The config.json

```json
{
	"write_only": false,
	"data_processor": [
		{
			"type": "direct",
			"args": {
				"world_path": "./world0.json"
			}
		},
		{
			"type": "direct",
			"args": {
				"world_path": "./world1.json"
			}
		}
	],
	"renderer": {
		"type": "basic_path_tracer",
		"args": {
			"iteration": 8,
			"rays_per_pixel": 100,
			"p_continue": 1.0,
			"n_air": 1.0
		}
	},
	"consumer": {
		"type": "image",
		"args": {
			"format": "png",
			"output_name": "./tracing.png"
		}
	}
}
```

## Data Process

### What should a `DataProcessor` do

- `new()` function to create it self
  - **It's not in the trait!**
  - **It should not have any arg, or the `Configuration` can't use it**
- `process()`: Use the config_path and the args to read in the world data.
  - Apart from the `Direct` processor, others have to figure out the way to fill the `World` struct.
- `write()`: write a `World` to the output path in its own way.
- ...(not in the trait)

### Add New `DataProcessor`

- create a new processor under the data_processor folder
- A struct impls `new()` and `DataProcessor` trait

### DataProcessors

#### Direct

- (De)serialize the world to(from) the `World` corresponding JSON directly.
- The modification of this is tricky.
  - Because it deserializes the data from the world directly, you have to change the way `World` is deserialized to change it.
  - no other things need to be changed

## World

- The inner protocol(representation) of the world data

### Structure

- camera (optional for deserialization)
- objects

### Methods

- `merge_world()`: merge the world from another world. Mostly for merge the results from multiple data processors.

### Objects

Structure

- name
- mesh
- material

#### Why `enum` not `dyn trait`

- For easier deserialization
- For easier access to the members, or you have to write `get/set` for all the members

### Mesh

**TODO**: Create a folder under the world folder for this, add serde.rs for this type

#### Add New Mesh Type

1. Write the new one (`Sphere { data: Box<SphereMesh> }`) inside the enum
2. Create a struct for this new mesh
3. Make sure the serde methods (add custom ones inside the serde.rs for now)

### Material

**TODO**: Create a folder under the world folder for this, add serde.rs for this type

- Texture should be specified in this type
  - If it's a path to the texture, then you have to write a custom serde to deserialize it to your inner data type
  - **Texture may be a separate enum** held by material

#### Add new Material Type

1. Write the new one (`Diffuse { data: Box<DiffuseMaterial> },`) inside the enum
2. Create a struct for this new material
3. Make sure the serde methods (add custom ones inside the serde.rs for now)
