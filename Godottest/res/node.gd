extends Node

var count = 0

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	play_next()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass


func play_next():
	print(count)
	get_next().start()
	count += 1
	
func prep_next():
	get_next().prep()

func get_next():

	match count % 6:
		0:
			return $VideoStreamPlayer
		1:
			return $VideoStreamPlayer2
		2:
			return $VideoStreamPlayer3
		3:
			return $VideoStreamPlayer4
		4:
			return $VideoStreamPlayer5
		5:
			return $VideoStreamPlayer6
