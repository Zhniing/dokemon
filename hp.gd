extends ProgressBar


var hp


func _ready():
	hp = $"../Stats".stats[0]
	max_value = hp
	value = max_value


func damage(x):
	hp = max(hp - x, 0)
	value = hp


func heal(x):
	hp = min(hp + x, $"../Stats".stats[0])
	value = hp
