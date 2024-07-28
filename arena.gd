extends Control


func _ready():
	var P1 = $Player1
	var P2 = $Player2

	# Inital HP bar
	var hp = $Player1/HP
	hp.max_value = $Player1.hero.stats[0]
	hp.value = hp.max_value
	# Bind move buttons
	var btns = $Player1/MoveBtns
	for btn in btns.get_children():
		btn.connect("pressed", Callable(btns, "attack").bind(P1, P2))

	# Inital HP bar
	hp = $Player2/HP
	hp.max_value = $Player2.hero.stats[0]
	hp.value = hp.max_value
	# Bind move buttons
	btns = $Player2/MoveBtns
	for btn in btns.get_children():
		btn.connect("pressed", Callable(btns, "attack").bind(P2, P1))
