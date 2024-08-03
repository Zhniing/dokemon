extends Button


@onready var P1 = $/root/Arena/Player1
@onready var P2 = $/root/Arena/Player2


func _ready():
	visible = false

	connect("pressed", func():
		# Hide this button
		visible = false

		# Clear the battle log
		$"/root/Arena/BattleLog".clear()

		# Reset the number of turns
		$/root/Arena.turn = 1
		$/root/Arena/Turn.text = "[center]回合 1[/center]"

		# Reset the HP
		P1.get_node("Hero").revive()
		P2.get_node("Hero").revive()

	)

func _process(delta):
	# Display this button
	if not visible && (P1.alive_num <= 0 || P2.alive_num <= 0):
		visible = true
