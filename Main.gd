extends Node2D

@export var falling_object_scene: PackedScene

var bananas: int = 0
var controller: int = 0

func _ready() -> void:
	var initialize_response: Dictionary = Steam.steamInitEx()
	print("Did Steam initialize?: %s " % initialize_response)
	Steam.inputInit(true)
	
	
	controller=Steam.getControllerForGamepadIndex(0)
		
	Steam.triggerRepeatedHapticPulse(controller, 0, 50000,50000,10,0)

	var inputType = Steam.getInputTypeForHandle(controller)
	$SteamStatus.text=str(controller)+" "+str(inputType)+" "+str(initialize_response)


	
	$Button.connect("pressed", Callable(self, "_on_button_pressed"))
	$SubViewportContainer/Label.text = "bananas: " + str(bananas)
	
	
	#var timer = Timer.new()
	#timer.wait_time = 1.0 # Set wait time to 2 seconds
	#timer.one_shot = false
	#add_child(timer)
	#timer.connect("timeout", Callable(self, "_on_Timer_timeout"))
	#timer.start() # Start the timer
#
#func _on_Timer_timeout():
	#print("Timer timed out, calling function...")
	#$SteamStatus.text=str(Steam.getConnectedControllers()) + " " + str(Steam.getInputTypeForHandle(Steam.getControllerForGamepadIndex(0)))



func _on_button_pressed():
	bananas += 1
	$SubViewportContainer/Label.text = "bananas: " + str(bananas)
	
	var instance = falling_object_scene.instantiate()
	
	instance.position = Vector2(randf_range(0, get_viewport_rect().size.x), 0)
	
	add_child(instance)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass


func _on_button_2_pressed() -> void:
	print ("button pressed")
	
	Steam.runFrame()
	
	var ctr=Steam.getControllerForGamepadIndex(0)
	
	$SteamStatus.text=str(Steam.getConnectedControllers()) + " " +ctr+" " + str(Steam.getInputTypeForHandle(Steam.getControllerForGamepadIndex(0)))


	Steam.triggerRepeatedHapticPulse(ctr, 0, 50000,50000,10,0)
	#var device := 0 # The joystick device index, change as needed
	#var weak_magnitude := float($Weak.text) # Weak vibration strength (0 to 1)
	#var strong_magnitude := float($Strong.text)  # Strong vibration strength (0 to 1)
	#var duration := float($Duration.text)  # Duration in seconds
#
	## Trigger joystick vibration
	#Input.start_joy_vibration(device, weak_magnitude, strong_magnitude, duration)
