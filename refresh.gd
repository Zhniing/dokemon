extends Button


@onready var P1 = $/root/Arena/Player1
@onready var P2 = $/root/Arena/Player2


func _ready():
	visible = false

	connect("pressed", func():
		# Hide this button
		visible = false

		# Clear the battle log
		$/root/Arena/BattleLog.text = ""

		# Reset the HP
		P1.hero.hp = P1.hero.stats[0]
		P1.get_node("HP").value = P1.hero.hp
		P2.hero.hp = P2.hero.stats[0]
		P2.get_node("HP").value = P2.hero.hp
	)

func _process(delta):
	# Display this button
	if not visible && (P1.hero.hp <= 0 || P2.hero.hp <= 0):
		visible = true
