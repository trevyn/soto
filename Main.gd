extends Node2D

@export var falling_object_scene: PackedScene

var bananas: int = 0

func _ready() -> void:
	$Button.connect("pressed", Callable(self, "_on_button_pressed"))
	$Label.text = "bananas: " +str(bananas)

func _on_button_pressed():
	bananas += 1
	$Label.text = "bananas: " +str(bananas)
	
	var instance = falling_object_scene.instantiate()
	
	instance.position = Vector2(randf_range(0, get_viewport_rect().size.x),0)
			
	add_child(instance)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass
