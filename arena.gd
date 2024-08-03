extends Control


var commands = {}
var turn = 1


func _ready():
	# Bind move buttons
	var btns = $Player1/Hero/MoveBtns
	for btn in btns.get_children():
		btn.connect("pressed", func():
			commands[$Player1/Hero] = Callable(btn, btn.move_name)
		)
	btns = $Player2/Hero/MoveBtns
	for btn in btns.get_children():
		btn.connect("pressed", func():
			commands[$Player2/Hero] = Callable(btn, btn.move_name)
		)


func execute_commands():
	if $Player1.alive_num <= 0 || $Player2.alive_num <= 0:
		return

	# Sort by the speed
	var order = commands.keys()
	order.sort_custom(func(a, b):
		return a.get_stat(5) > b.get_stat(5)
	)

	# Write the number of turns to the battle log
	if $Player1/Hero/HP.hp > 0 && $Player2/Hero/HP.hp > 0:
		$/root/Arena/BattleLog.text += "回合 " + str(turn) + "\n"

	# Execute the commands sequentially
	for i in order:
		commands[i].call()

	# Update the number of turns
	turn += 1
	$/root/Arena/Turn.text = "[center]回合 " + str(turn) + "[/center]"

	commands.clear()


func _process(delta):
	if commands.size() == 2:
		execute_commands()
