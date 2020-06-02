extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var wanna_act : bool = false;
var is_acting : bool = false;
var is_aiming : bool = false;
var aim_distance : float = 0;

var ka = 100;
var kb = 25;
var Eact = 0.2;

onready var player : RigidBody = $".."
onready var aim : RayCast = $"../Aim"
onready var pointer : Spatial = $"../Pointer"
var target : Node = null;
var target_point : Vector3 = Vector3.ZERO;
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _unhandled_input(event):
	if event.is_action("player_action"):
		wanna_act = event.is_action_pressed("player_action");
		get_tree().set_input_as_handled();

func _physics_process(delta):
	
	if not wanna_act:
		is_aiming = aim.is_colliding();
		if is_aiming:
			pointer.translation = player.to_local(aim.get_collision_point())
	
	if wanna_act:
		if is_aiming and not is_acting:
			target = aim.get_collider();
			if target and target.is_class("RigidBody"):
				is_acting = true;
				target_point = target.to_local(aim.get_collision_point());
				aim_distance = aim.to_local(aim.get_collision_point()).length();
	else:
		is_acting = false;
	
	if is_acting:
		pointer.translation = aim.translation-aim.transform.basis.z*aim_distance;
		var delta_position : Vector3 = (pointer.to_global(Vector3.ZERO) - target.to_global(target_point));
		var direction : Vector3 = delta_position.normalized();
		var distance = delta_position.length();
		var local_velocity : Vector3 = target.linear_velocity + target.angular_velocity.cross(target.to_global(target_point) - target.to_global(Vector3.ZERO));
		var target_velocity = delta_position;
		var force = Vector3.UP*target.weight + ka*delta_position + kb*(target_velocity - local_velocity);
		target.apply_impulse(target.to_global(target_point) - target.to_global(Vector3.ZERO), force*delta);
