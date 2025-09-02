import React, {useState, useEffect, useRef} from "react";
import {View, Text, StyleSheet, Image, TouchableOpacity, SafeAreaView} from "react-native";
import Swiper from "react-native-swiper";
import {Ionicons} from '@expo/vector-icons';
import Colors from '../constants/colors';

const PlacesScreen = () => {

    const [places, setPlaces] = useState([]);
    const [allCardsSwiped, setAllCardsSwiped] = useState(false); 
    const swiperRef = useRef(null);

    useEffect(() => {
        fetch('http://172.20.10.2:3000/places')
            .then(response =>response.json())
            .then((data) => {
                setPlaces(data);
                console.log("Fetched places data: ", data);
                setAllCardsSwiped(data.length === 0); 
            })
            .catch((error) => console.error("Error fetching places:", error));
    }, []);

    const handleLike = (cardIndex) => {
        const place = places[cardIndex];
        if (!place) {
            console.warn("handleLike received an undefined place at index:", cardIndex);
            return;
        }
        fetch(`http://172.20.10.2:3000/like/${place.id}`, {
            method: 'POST',
        }).catch((error) => console.error("Error liking place:", error));
    };

    const renderCard = (card) => {
        if (!card) {
            console.warn("renderCard received an undefined card!");
            return null;
        }


        const { image = '', name = 'N/A', price = 0, location = 'N/A', description = 'No description provided' } = card;

        return (
            <View style={styles.card}>
                <Image source={{ uri: image || '' }} style={styles.cardImage}/>
                <View style={styles.cardTextContainer}>
                    <Text style={styles.cardTitle}>{name}, ${price}</Text>
                    <Text style={styles.cardLocation}>{location}</Text>
                    <Text style={styles.cardDescription}>{description}</Text>
                </View>
            </View>
        );
    };

    const NoMoreCards = () => {
        return (
            <View style={styles.noMoreCardsContainer}>
                <Text style={styles.noMoreCardsTitle}>No more places left to show!</Text>
                <Text style={styles.noMoreCardsText}> Check back later!!</Text>
            </View>
        );
    };


    return (
        <SafeAreaView style={styles.container}>
            <View style={styles.swiperWrapper}>
                {places.length > 0 && !allCardsSwiped ? (
                    <Swiper
                        key={places.length} // force remount on data change
                        ref={swiperRef}
                        cards={places}
                        renderCard={renderCard}
                        onSwipedRight={handleLike}
                        cardIndex={0}
                        backgroundColor={'transparent'}
                        stackSize={3}
                        infinite={false}
                        loop={false}
                        showSecondCard={true}
                        animateCardOpacity
                        onSwipedAll={() => setAllCardsSwiped(true)} 
                        overlayLabel={{
                            left: {
                              title: <Text style={styles.nopeLabel}>NOPE</Text>,
                              style: {label: styles.nopeLabel, wrapper: styles.overlayWrapper},
                            },
                            right: {
                              title: <Text style={styles.likeLabel}>LIKE</Text>,
                              style: {label: styles.likeLabel, wrapper: styles.overlayWrapper},
                            },
                          }}
                          
                    />
                ) : (
                    <NoMoreCards/>
                )}
                {places.length > 0 && !allCardsSwiped && (
                    <View style={styles.buttonContainer}>
                        <TouchableOpacity style={[styles.button, styles.nopeButton]} onPress={() => swiperRef.current.swipeLeft()}>
                            <Ionicons name="close" size={32} color={Colors.danger}/>
                        </TouchableOpacity>
                        <TouchableOpacity style={[styles.button, styles.likeButton]} onPress={() => swiperRef.current.swipeRight()}>
                            <Ionicons name="heart" size={32} color={Colors.secondary}/>
                        </TouchableOpacity>
                    </View>
                )}
            </View>
        </SafeAreaView>
    );

};

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: Colors.lightGray,
    },

    swiperWrapper: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
    },

    card: {
        flex: 0.75, 
        borderRadius: 15,
        shadowRadius: 5,
        shadowColor: Colors.black,
        shadowOpacity: 0.1,
        shadowOffset: {width: 0, height: 2},
        justifyContent: 'center',
        alignItems: 'center',
        backgroundColor: Colors.white,
        elevation: 3,
    },

    cardImage: {
        width: '100%',
        height: '70%',
        borderTopLeftRadius: 15,
        borderTopRightRadius: 15,
    },

    cardTextContainer: {
        padding: 15,
    },

    cardTitle: {
        fontSize: 22,
        fontWeight: 'bold',
    },

    cardLocation: {
        fontSize: 16,
        color: Colors.gray,
        marginTop: 5,
    },

    cardDescription: {
        fontSize: 14,
        color: '#444',
        marginTop: 10,
    },

    buttonContainer: {
        flexDirection: 'row',
        justifyContent: 'space-evenly',
        alignItems: 'center',
        paddingVertical: 10,
        position: 'absolute',
        bottom: 0,
        width: '100%',
        backgroundColor: Colors.lightGray,
    },

    button: {
        width: 64,
        height: 64,
        borderRadius: 32,
        backgroundColor: Colors.white,
        justifyContent: 'center',
        alignItems: 'center',
        shadowColor: Colors.black,
        shadowOffset: { width: 0, height: 1 },
        shadowOpacity: 0.2,
        shadowRadius: 3,
        elevation: 2,
    },
    
    nopeButton: {
        borderColor: Colors.danger,
        borderWidth: 2,
    },

    likeButton: {
        borderColor: Colors.secondary,
        borderWidth: 2,
    },

    overlayWrapper: {
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
    },

    likeLabel: {
        fontSize: 45,
        fontWeight: 'bold',
        color: Colors.secondary,
        borderColor: Colors.secondary,
        borderWidth: 2,
        padding: 10,
        borderRadius: 10,
        transform: [{ rotate: '-15deg' }],
    },

    nopeLabel: {
        fontSize: 45,
        fontWeight: 'bold',
        color: Colors.danger,
        borderColor: Colors.danger,
        borderWidth: 2,
        padding: 10,
        borderRadius: 10,
        transform: [{ rotate: '15deg' }],
    },

    noMoreCardsContainer: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
    },

    noMoreCardsTitle: {
        fontSize: 24,
        fontWeight: 'bold',
        color: Colors.gray,
    },

    noMoreCardsText: {
        fontSize: 22,
        fontWeight: 'bold',
        color: Colors.gray,
    },
    
})

export default PlacesScreen;