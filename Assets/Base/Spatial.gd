tool
extends Spatial

# VisualServer expects references to be kept around
export var mesh : Resource

func _ready():
	# Create a visual instance (for 3D).
	var instance = VisualServer.instance_create()
	# Set the scenario from the world, this ensures it
	# appears with the same objects as the scene.
	var scenario = self.get_world().scenario
	VisualServer.instance_set_scenario(instance, scenario)
	# Add a mesh to it.
	# Remember, keep the reference.
	mesh = load("res://mesh.obj")

	VisualServer.instance_set_base(instance, mesh.get_rid())
	# Move the mesh around.
	var xform = Transform(Basis(), Vector3(0, 0, 0))
	VisualServer.instance_set_transform(instance, xform)
	print ("gdscript mesh: ", mesh);
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
