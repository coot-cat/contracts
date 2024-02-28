## CootCat Game

### Design

##### Contracts

```plantuml
struct Game {
  cat_contract: Addr
  item_contract: Addr

  mint_cat(): void
  feed_cat(): void
  clean_cat(): void
  upgrade_cat(): void
  checkin(): void
}

note top: Entry Contract

struct Cat {
  admin: Addr
  owner: Addr
  name: String
  mint(): void
}

note left of Cat::mint()
  only Game Contract
  can invoke through mint_cat()
end note

struct Gear {
  admin: Addr
  owner: Addr
  name: String,
  type: String
  mint(): void
}

note right of Gear::mint()
  only Game Contract
  can invoke through checkin()
end note

  Cat <- Game
  Game -> Gear
```

##### Sequence Diagram

```plantuml
== Mint ==
Game -> Cat: Mint Cat
Cat --> Game

== Upgrade Cat ==
Game -> Game: Upgrade Cat (consume upgrade item)
Game -> Cat: Check Cat info
Cat --> Game: Cat info
Game -> Gear: Check Gear info
Gear --> Game: Gear info
Game -> Cat: upgrade
Game -> Gear: burn
```
