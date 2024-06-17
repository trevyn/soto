extends Node2D

var bananas: int = 0

func _ready() -> void:
	$Button.connect("pressed", Callable(self, "_on_button_pressed"))
	$Label.text = "bananas: " +str(bananas)

func _on_button_pressed():
	bananas += 1
	$Label.text = "bananas: " +str(bananas)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
