package main

type option struct {
	Description     string
	Result          string
	AlreadyDone     bool
	ImportantResult string
}

type room struct {
	Name          string
	Description   string
	Options       [3]option
	AdjacentRooms [4]string
}

func fetchRooms() []room {
	return []room{
		{
			Name:        "Living Room",
			Description: "This is the room where you just chill out and relax. It has a couch and a tv in it.",
			Options: [3]option{
				{
					Description:     "Sit on the couch",
					Result:          "You find a ton of loose change in between the cushions, adding up to one GP! Score!",
					AlreadyDone:     false,
					ImportantResult: "gain money",
				},
				{
					Description:     "Turn on the TV",
					Result:          "You are electrocuted! You lose 10 health :( Guess that TV must have been pretty old.",
					AlreadyDone:     false,
					ImportantResult: "lose health",
				},
				{
					Description: "Investigate the shadowy, cobwebby corner...",
					Result:      "There are shadows. And cobwebs. And probably a harmless spider somewhere. Yea, that's about it.",
					AlreadyDone: false,
				},
			},
			AdjacentRooms: [4]string{"Office"},
		},
		{
			Name:        "Office",
			Description: "This is a good place to work and study in peace. There's a desk with a laptop on it, and some filing cabinets.",
			Options: [3]option{
				{
					Description:     "Use computer",
					Result:          "You program a fun thing and it fills your soul with joy! You gain 10 health.",
					AlreadyDone:     false,
					ImportantResult: "gain health",
				},
				{
					Description: "Sit in awesome swivel-chair",
					Result:      "You have the time of your life, but you get a little dizzy spinning around.",
					AlreadyDone: false,
				},
				{
					Description:     "Search through the filing cabinet",
					Result:          "You find many boring documents, but --Aha! You find a wad of cash totalling up to 1 GP!",
					AlreadyDone:     false,
					ImportantResult: "gain money",
				},
			},
			AdjacentRooms: [4]string{
				"Living Room",
				"Mud Room",
				"Hallway",
			},
		},
		{
			Name:        "Mud Room",
			Description: "This is a pretty small room, just for keeping your muddy rainboots and a few other supplies.",
			Options: [3]option{
				{
					Description:     "Try on muddy rainboots",
					Result:          "As you put on the rainboots, they start to glow with a strange light! Sorcery! Lose 10 health.",
					AlreadyDone:     false,
					ImportantResult: "lose health",
				},
				{
					Description: "Rifle through the cabinets",
					Result:      "Ooooh, a bucket of mealworms! You pull out some worm feed and feed the worms. That's about it.",
					AlreadyDone: false,
				},
				{
					Description: "Throw an unexpected dance party",
					Result:      "Unexpectedly, you start dancing and showing off your best moves. It's a fun time.",
					AlreadyDone: false,
				},
			},
			AdjacentRooms: [4]string{
				"Office",
				"Backyard",
			},
		},
		{
			Name:        "Backyard",
			Description: "Ah, fresh air! This is a moderately sized backyard covered in a grassy lawn, and bordered by a forest of trees.",
			Options: [3]option{
				{
					Description:     "Build something out of rocks",
					Result:          "You build an awesome sculpture, but in all that excitement you lose a Gold Piece (assuming that you have one to lose).",
					AlreadyDone:     false,
					ImportantResult: "lose money",
				},
				{
					Description:     "Frolick around and do fun things",
					Result:          "You do lots of fun things, and overall have a grand old time. You are so rejuvinated that you gain 10 health.",
					AlreadyDone:     false,
					ImportantResult: "gain health",
				},
				{
					Description:     "Investigate the dark, mysterious woods...",
					Result:          "There are a bunch of trees. Not much else to... --oh no! An angry radioactive squirrel leaps from a nearby tree and bites you. You lose 10 health.",
					AlreadyDone:     false,
					ImportantResult: "lose health",
				},
			},
			AdjacentRooms: [4]string{
				"Mud Room",
			},
		},
	}
}