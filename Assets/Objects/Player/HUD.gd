extends Control


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
onready var debug_label = $Debug/Label;
onready var fps_label = $FPS/Label;
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	debug_label.text = "Pointer position " + String($"../Pointer".translation) + "\n";
	debug_label.text = "Player is acting: " + String($"../ActionController".is_acting);
	fps_label.text = String(Engine.get_frames_per_second()) + "fps";
	pass
