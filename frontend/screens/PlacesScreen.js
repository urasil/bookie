import React, { useState, useEffect, useRef } from "react";
import { 
  View, 
  Text, 
  StyleSheet, 
  ImageBackground, 
  TouchableOpacity, 
  SafeAreaView 
} from "react-native";
import TinderCard from "react-tinder-card";
import { Ionicons } from "@expo/vector-icons";
import Colors from "../constants/colors"; 

const PlacesScreen = () => {
  const [places, setPlaces] = useState([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const childRef = useRef(null); 

  useEffect(() => {
    fetch("http://localhost:3000/places?location=40.7128,-74.0060&type=restaurant")
      .then((res) => res.json())
      .then((data) => {
        if (Array.isArray(data)) {
          setPlaces(data);
          setCurrentIndex(0);
        } else {
          setPlaces([]);
        }
      })
      .catch((err) => console.error("Error fetching places:", err));
  }, []);

  const swiped = (direction, place) => {
    console.log(`Swiped ${direction} on ${place.id}`);
    if (direction === "right") {
      fetch(`http://localhost:3000/like/${place.id}`, { 
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ user_id: 'test_user', place: place}),
      })
        .catch((err) => console.error("Error liking place:", err));
    }
    setCurrentIndex((prevIndex) => prevIndex + 1);
  };

  const outOfFrame = (placeId) => {
    console.log(`${placeId} left the screen`);
  };

  const swipe = async (dir) => {
    if (currentIndex < places.length && childRef.current) {
      await childRef.current.swipe(dir);
    }
  };

  const currentPlace = places[currentIndex];

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.cardContainer}>
        {currentPlace ? (
          <TinderCard
            ref={childRef}
            key={currentPlace.id}
            onSwipe={(dir) => swiped(dir, currentPlace)}
            onCardLeftScreen={() => outOfFrame(currentPlace.id)}
            preventSwipe={["up", "down"]}
            containerStyle={styles.tinderCardWrapper}
          >
            <ImageBackground
              source={{ uri: currentPlace.image }}
              style={styles.card}
              imageStyle={{ borderRadius: 20 }} 
            >
              {/* The overlay with text and buttons goes directly inside */}
              <View style={styles.overlay}>
                <View style={styles.info}>
                  <Text style={styles.title}>
                    {currentPlace.name || currentPlace.title || "Unnamed"}
                  </Text>
                  <Text style={styles.description}>
                    {currentPlace.description || "No description"}
                  </Text>
                </View>
                <View style={styles.cardButtons}>
                  <TouchableOpacity style={styles.button} onPress={() => swipe("left")}>
                    <Ionicons name="close" size={40} color={Colors.white} />
                  </TouchableOpacity>
                  <TouchableOpacity
                    style={[styles.button, styles.like]}
                    onPress={() => swipe("right")}
                  >
                    <Ionicons name="heart" size={40} color={Colors.white} />
                  </TouchableOpacity>
                </View>
              </View>
            </ImageBackground>
          </TinderCard>
        ) : (
          <View style={styles.noMoreCards}>
            <Text style={styles.noMoreText}>No more places left!</Text>
          </View>
        )}
      </View>
    </SafeAreaView>
  );
};

export default PlacesScreen;

// *******STYLES*******
const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f2f2f2',
  },
  cardContainer: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
  },
  tinderCardWrapper: {
    width: '85%',
    height: '85%',
  },
  card: {
    flex: 1, 
    justifyContent: 'flex-end', // Pushes the overlay to the bottom
    shadowColor: Colors.black,
    shadowOpacity: 0.15,
    shadowRadius: 10,
    shadowOffset: { width: 0, height: 5 },
    elevation: 5,
  },
  overlay: {
    // This view sits on top of the image and holds the text/buttons
    backgroundColor: 'rgba(0,0,0,0.5)',
    padding: 10,
    borderBottomLeftRadius: 20, 
    borderBottomRightRadius: 20,
  },
  info: {},
  title: {
    fontSize: 24,
    fontWeight: "bold",
    color: Colors.white,
  },
  description: {
    marginTop: 5,
    fontSize: 16,
    color: Colors.white,
  },
  cardButtons: {
    flexDirection: "row",
    justifyContent: "space-evenly",
    marginTop: 20,
    width: '100%',
  },
  button: {
    backgroundColor: Colors.danger,
    width: 70,
    height: 70,
    borderRadius: 35,
    justifyContent: "center",
    alignItems: "center",
  },
  like: {
    backgroundColor: Colors.secondary,
  },
  noMoreCards: {
    justifyContent: "center",
    alignItems: "center",
  },
  noMoreText: {
    fontSize: 22,
    fontWeight: "bold",
    color: Colors.gray,
  },
});
