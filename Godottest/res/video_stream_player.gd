extends VideoStreamPlayer

var fade_direction = 0.02

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	self.connect("finished", Callable(self, "_on_video_finished"))
	self.play()
	#self.paused=1
	
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	#modulate.a+=fade_direction
	if modulate.a > 1.0 || modulate.a < 0.0: fade_direction=-fade_direction
	pass

func _on_video_finished():
	self.get_parent().play_next()
