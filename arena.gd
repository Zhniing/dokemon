extends Control


var commands = {}
var turn = 1


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
		btn.connect("pressed", func():
			commands[P1.name] = Callable(btns, "attack").bind(P1, P2)
		)

	# Inital HP bar
	hp = $Player2/HP
	hp.max_value = $Player2.hero.stats[0]
	hp.value = hp.max_value
	# Bind move buttons
	btns = $Player2/MoveBtns
	for btn in btns.get_children():
		btn.connect("pressed", func():
			commands[P2.name] = Callable(btns, "attack").bind(P2, P1)
		)

func _process(delta):
	if commands.size() == 2:
		# Sort by the speed
		var order = commands.keys()
		order.sort_custom(func(a, b):
			return get_node(a).hero.stats[5] > get_node(b).hero.stats[5]
		)

		# Write the number of turns to the battle log
		$/root/Arena/BattleLog.text += "回合 " + str(turn) + "\n"

		# Execute the commands
		for i in order:
			commands[i].call()

		commands.clear()

		# Update the number of turns
		if $Player1.hero.hp > 0 && $Player2.hero.hp > 0:
			turn += 1
			$/root/Arena/Turn.text = "[center]回合 " + str(turn) + "[/center]"
