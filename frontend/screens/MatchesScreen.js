import React, {useState, useCallback} from "react";
import {View, Text, StyleSheet, FlatList, Image, SafeAreaView, RefreshControl, TouchableOpacity} from 'react-native';
import {useFocusEffect} from '@react-navigation/native';
import {Ionicons} from '@expo/vector-icons';
import Colors from '../constants/colors';

const MatchesScreen = () =>{
    const [matches, setMatches] = useState([]);
    const [refreshing, setRefreshing] = useState(false);

    const fetchMatches = () => {
        setRefreshing(true);
        fetch('http://172.20.10.2:3000/matches')
            .then((response) => response.json())
            .then((data) => setMatches(data))
            .catch((error) => console.error("Error fetching matches: ", error))
            .finally(() => setRefreshing(false));
    };
    
    // this will refetch data every time the screen comes into view
    useFocusEffect(
        useCallback(() => {
            fetchMatches();
        }, [])
    );

    const renderItem = ({item}) => {
        if (!item) {
            console.warn("renderItem received an undefined item!");
            return null;
        }
        return (
            <TouchableOpacity style={styles.itemContainer} activeOpacity={0.7}>
                <Image source={{uri: item.image}} style={styles.itemImage}/>
                <View style={styles.itemTextContainer}>
                    <Text style={styles.itemName}>{item.name}</Text>
                    <Text style={styles.itemLocation}>${item.price}   {item.location}</Text>
                </View>
                <Ionicons name="chevron-forward" size={24} color={Colors.gray}/>
            </TouchableOpacity>
        );
    };

    const EmptyListComponent = () => (
        <View style={styles.emptyContainer}>
            <Ionicons name="heart-dislike-outline" size={64} color={Colors.lightGray}/>
            <Text style={styles.emptyTitle}>No Matches Yet - Keep Swiping!</Text>
            <Text style={styles.emptySubtitle}>Swipte right on places you like to them here!</Text>
        </View>
    );

    return (
        <SafeAreaView style={styles.container}>
            <Text style={styles.header}>Your Matches</Text>
            <FlatList
                data={matches}
                renderItem={renderItem}
                keyExtractor={(item) => item.id}
                contentContainerStyle={{flexGrow: 1}}
                ListEmptyComponent={EmptyListComponent}
                refreshControl={<RefreshControl refreshing={refreshing} onRefresh={fetchMatches}/>}
            />
        </SafeAreaView>
    );
};

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: Colors.white,
    },
    
    header: {
        fontSize: 32,
        fontWeight: 'bold',
        paddingHorizontal: 20,
        paddingTop: 20,
        paddingBottom: 10,
    },
    
    itemContainer: {
        flexDirection: 'row',
        paddingVertical: 12,
        paddingHorizontal: 20,
        alignItems: 'center',
        backgroundColor: Colors.white,
    },
    
    itemImage: {
        width: 70,
        height: 70,
        borderRadius: 35,
        marginRight: 15,
    },

    itemTextContainer: {
        flex: 1,
    },

    itemName: {
        fontSize: 18,
        fontWeight: '600',
    },

    itemLocation: {
        fontSize: 14,
        color: Colors.gray,
        marginTop: 4,
    },

    emptyContainer: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
        padding: 20,
    },

    emptyTitle: {
        fontSize: 22,
        fontWeight: 'bold',
        marginTop: 16, 
        color: '#555',
    },

    emptySubtitle: {
        fontSize: 16,
        color: Colors.gray,
        textAlign: 'center',
        marginTop: 8,
    },
});

export default MatchesScreen;