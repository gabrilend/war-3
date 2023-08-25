def card_test():
    # Create a new card
    card = Card()
    # Set the card's value
    card.set_value(10)

    # Assert that the card's value is 10
    assert card.get_value() == 10

    assert card.resolve() == 10
