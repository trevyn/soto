extends Node2D

var alpha = 0

# Function to set the alpha for the parent and its children
func set_alpha(alpha_value: float) -> void:
	# Ensure alpha_value is between 0 and 1
	alpha_value = clamp(alpha_value, 0.0, 1.0)
	
	# Set the parent's alpha if it has a Modulate property
	if has_method("set_modulate"):
		modulate = Color(modulate.r, modulate.g, modulate.b, alpha_value)
	
	# Loop through each child
	for child in get_children():
		if child.has_method("set_modulate"):
			var child_modulate = child.modulate
			child.modulate = Color(child_modulate.r, child_modulate.g, child_modulate.b, alpha_value)

# Example function to change alpha
func _ready():
	pass
	#set_alpha(0.5) # Change all children's alpha to 0.5

func _process(_delta):
	
	if alpha < 3.0:
		alpha += 0.006;
		set_alpha(alpha-2.0) # Change all children's alpha to 0.5
