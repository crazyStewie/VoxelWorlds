extends Camera


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var speed : float = 10;
var target_pitch : float = 0;
var target_yaw : float = 0;
var sensitivity : float = 1;
# Called when the node enters the scene tree for the first time.
func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED);
	$"../VoxelWorld".set_origin(self);
	pass # Replace with function body.

func _unhandled_input(event):
	if event is InputEventMouseMotion:
		target_pitch -= event.relative.y*sensitivity;
		target_yaw -= event.relative.x*sensitivity;
		target_pitch = clamp(target_pitch, -89, 89);
		target_yaw = wrapf(target_yaw, -180, 180);
		self.rotation_degrees.x = target_pitch;
		self.rotation_degrees.y = target_yaw;


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta : float):
	self.translation += -delta*(Input.get_action_strength("player_forward") - Input.get_action_strength("player_backward"))*self.transform.basis.z*speed;
	self.translation += -delta*(Input.get_action_strength("player_left") - Input.get_action_strength("player_right"))*self.transform.basis.x*speed;
	pass
