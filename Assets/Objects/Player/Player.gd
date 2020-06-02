extends RigidBody


export var player_speed : float = 10;
export var view_sensibility : float = 0.005;
var target_yaw : float = 0;
var has_clicked : bool = false;
var power : float = 20;

var is_jumping : bool = false;
var can_jump : bool = false;
var jump_timer : float = 0;
var jump_speed : float = 10;
var jump_duration : float = 0.5;

var max_accel = 10;
var kmov = 50;

var kjump = 250;
var foot_lenght = 1;
var foot_equilibrium_position = 0.8;
func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED);
	$Aim/Camera.current = true;
	pass # Replace with function body.

func handle_mouse_motion(event):
	var pitch = $Aim.rotation.x;
	pitch -= event.relative.y * view_sensibility;
	pitch = clamp(pitch,-PI/2 + 0.01, PI/2 - 0.01 );
	$Aim.rotation.x = pitch;
	$Aim.rotation.y -=  event.relative.x * view_sensibility;

func _unhandled_input(event):
	if event is InputEventMouseMotion:
		handle_mouse_motion(event);

func handle_movement(delta : float):
	var forward : Vector3 = -$Aim.global_transform.basis.z;
	forward.y = 0;
	forward = forward.normalized();
	var right = get_perpendicular_vector(forward, Vector3.UP);
	var velocity : Vector3 = Vector3.ZERO;
	var target_movement : Vector3 = Vector3.ZERO;
	var moving : bool = false;
	if Input.is_action_pressed("player_forward"):
		target_movement += forward;
		moving = true;
	if Input.is_action_pressed("player_backward"):
		target_movement -= forward;
		moving = true;
	if Input.is_action_pressed("player_right"):
		target_movement += right;
		moving = true;
	if Input.is_action_pressed("player_left"):
		target_movement -= right;
		moving = true;
	if target_movement.length() > 1.0:
		target_movement = target_movement.normalized()
		moving = true;
	if moving:
		velocity.x = target_movement.x*player_speed;
		velocity.z = target_movement.z*player_speed;
	var delta_velocity : Vector3 = velocity - self.linear_velocity;
	delta_velocity.y = 0;
	var acceleration : Vector3 = kmov*delta_velocity;
	if acceleration.length() > max_accel:
		acceleration = acceleration.normalized()*max_accel;
	self.add_central_force(acceleration*mass);
	
func is_on_floor():
	return $Foot.is_colliding()

func handle_jump(delta: float):
	if self.is_on_floor() and not is_jumping:
		can_jump = true;
		is_jumping = false;
		jump_timer = 0;
		var up_force : Vector3 = - self.mass*gravity_scale*PhysicsServer.body_get_direct_state(self.get_rid()).total_gravity
		var leg_delta = ($Foot.translation - to_local($Foot.get_collision_point())).length() - foot_equilibrium_position;
		var correcting_force = -(leg_delta+0.1*self.linear_velocity.y)*kjump*self.global_transform.basis.y;
		correcting_force *= mass;
		up_force += correcting_force;
		self.add_central_force(up_force);
	else:
		can_jump = false;
	
	if Input.is_action_just_pressed("player_jump") and can_jump:
		self.apply_central_impulse(Vector3.UP*mass*jump_speed);
		is_jumping = true;
		can_jump = false;
	
	if Input.is_action_pressed("player_jump") and is_jumping:
		self.add_central_force(-0.8*self.gravity_scale*mass*PhysicsServer.body_get_direct_state(self.get_rid()).total_gravity);
		is_jumping = true;
	
	if (not Input.is_action_pressed("player_jump")) or jump_timer > jump_duration:
		is_jumping = false;
	if is_jumping:
		jump_timer+=delta;

func get_perpendicular_vector(vector_a : Vector3, vector_b: Vector3 = Vector3(1,0,0)):
	var result = vector_a.cross(vector_b);
	if result.is_equal_approx(Vector3.ZERO):
		result = get_perpendicular_vector(vector_a, Vector3(0,1,0));
	return result.normalized();

func handle_pointer():
	if not $Aim.is_colliding():
		$Pointer.visible = false;
		return;
	$Pointer.visible = true;
	var new_position = self.to_local($Aim.get_collision_point())
	var vector_z = to_local($Aim.get_collision_normal()) - to_local(Vector3.ZERO);
	var vector_x = get_perpendicular_vector(vector_z);
	var vector_y = get_perpendicular_vector(vector_z, vector_x);
	var new_basis = Basis(vector_x, vector_y, vector_z);
	$Pointer.transform = Transform(new_basis, new_position);

func handle_action(delta):
	if Input.is_action_just_pressed("player_action") and $Aim.is_colliding():
		var target = $Aim.get_collider();
		if target.has_method("apply_impulse"):
			target.apply_impulse($Aim.get_collision_point()-target.to_global(Vector3.ZERO), -$Aim.get_collision_normal()*power);

func _physics_process(delta):
	handle_movement(delta);
	handle_jump(delta);
	handle_pointer();
	handle_action(delta);
	
