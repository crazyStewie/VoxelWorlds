extends Node

var player_state : PhysicsDirectBodyState;

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

export var player_speed : float = 4;
export var kmov : float = 10;
export var view_sensibility : float = 0.02;

export var kjump : float = 80;
export var Ejump : float = 0.1;
export var leg_rest : float = 0.6;
export var max_jump_duration : float = 0.3;
export var jump_speed : float = 5;
export var jump_effect : float = 0.6;

onready var aim : RayCast = $"../Aim";
onready var foot : RayCast = $"../Foot";
onready var player : RigidBody = $"..";

var target_movement : Vector3 = Vector3.ZERO;
var wanna_jump : bool = false;
var is_jumping : bool = false;
var can_jump : bool = false;
var jump_timer : float = 0;


onready var target_yaw : float = $"../..".rotation.y;
var target_pitch : float = 0;
var pitch_limit : float = PI/2;

var is_sprinting : bool = false;
var sprint_factor : float = 2;

# Called when the node enters the scene tree for the first time.
func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED);
	pass # Replace with function body.

func handle_movement_input(event : InputEvent):
	var event_handled : bool = false;
	if event.is_action("player_forward"):
		target_movement.z = - event.get_action_strength("player_forward");
		event_handled = true;
	elif event.is_action("player_backward"):
		target_movement.z = event.get_action_strength("player_backward");
		event_handled = true;
	elif event.is_action("player_right"):
		target_movement.x = event.get_action_strength("player_right");
		event_handled = true;
	elif event.is_action("player_left"):
		target_movement.x = - event.get_action_strength("player_left");
		event_handled = true;
	elif event.is_action("player_jump"):
		wanna_jump = event.is_action_pressed("player_jump");
		event_handled = true;
	elif event.is_action("player_sprint"):
		is_sprinting = event.is_action_pressed("player_sprint");
		event_handled = true;
	if event_handled:
		self.get_tree().set_input_as_handled()

func handle_mouse_capture(event: InputEvent):
	if event.is_action_pressed("ui_cancel"):
		if Input.get_mouse_mode() == Input.MOUSE_MODE_VISIBLE:
			Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED);
		else:
			Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE);

func handle_camera_input(event: InputEvent):
	if event is InputEventMouseMotion and Input.get_mouse_mode() == Input.MOUSE_MODE_CAPTURED:
		var mouse_movement : Vector2 = event.relative*view_sensibility;
		print(event.relative);
		print(mouse_movement);
		target_yaw -= mouse_movement.x;
		target_pitch -= mouse_movement.y;
		target_pitch = clamp(target_pitch, -pitch_limit, pitch_limit);

func _unhandled_input(event: InputEvent):
	handle_movement_input(event);
	handle_mouse_capture(event);
	handle_camera_input(event);

func handle_movement(delta: float):
	var planar_velocity : Vector3 = player_state.linear_velocity;
	planar_velocity.y = 0;
	var target_velocity = target_movement.rotated(Vector3.UP, target_yaw);
	if target_velocity.length_squared() > 1:
		target_velocity = target_velocity.normalized();
	target_velocity *= player_speed;
	if (is_sprinting):
		target_velocity *= sprint_factor;
	player_state.add_central_force(10*(target_velocity - planar_velocity));
	
	var weight = player.mass*player_state.total_gravity
	var up_force : Vector3 = Vector3.ZERO;
	if foot.is_colliding():
		var compression : float = (leg_rest - (foot.get_collision_point() - foot.to_global(Vector3.ZERO)).length());
		up_force = Vector3.UP*(kjump*(compression - Ejump*player_state.linear_velocity.y)) - weight;
		can_jump = true;
	else:
		can_jump = false;
	if (not wanna_jump) or jump_timer > max_jump_duration:
		jump_timer = 0;
		is_jumping = false;
		wanna_jump = false;
	if (can_jump and wanna_jump and not is_jumping):
		is_jumping = true;
		player_state.apply_central_impulse((jump_speed - player_state.linear_velocity.y)*player.mass*Vector3.UP);
	if (is_jumping):
		up_force = -jump_effect*weight;
		jump_timer += delta;
	
	player_state.apply_central_impulse(delta*(up_force));
	
func _physics_process(delta: float):
	player_state = PhysicsServer.body_get_direct_state($"..".get_rid());
	handle_movement(delta);

func _process(delta: float):
	aim.rotation.x = lerp(aim.rotation.x, target_pitch, 0.5);
	aim.rotation.y = lerp(aim.rotation.y, target_yaw, 0.5);
