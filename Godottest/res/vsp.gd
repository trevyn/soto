extends VideoStreamPlayer

@export var fadeout: bool = true
@export var fadein: bool = true
@export var streamlength: float = 10.

func _ready() -> void:
	self.connect("finished", Callable(self, "_on_video_finished"))
	self.play()
	self.paused=1
	#if fadein:
	modulate.a = 0

func prep():
	modulate.a = 1

func start():
	self.paused=0
	if fadein == false:
		modulate.a = 1

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if fadein && self.paused==false:
		if modulate.a < 1.0:
			modulate.a += 0.03
	
	if fadeout == false && self.stream_position > streamlength/2:
		get_parent().prep_next()
	
	if fadeout && self.stream_position > streamlength/2:
		if self.stream_position > streamlength - 0.5:
			modulate.a -= 0.05
			if modulate.a <= 0:
				stop()
				play()
				paused=1
				get_parent().play_next()
		else:
			modulate.a=1

	pass

func _on_video_finished():
	modulate.a = 0
	self.stop()
	self.play()
	self.paused=1
	self.get_parent().play_next()
