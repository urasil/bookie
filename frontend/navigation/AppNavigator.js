import React from 'react';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import {NavigationContainer} from '@react-navigation/native';
import {Ionicons} from '@expo/vector-icons';

import PlacesScreen from '../screens/PlacesScreen';
import MatchesScreen from '../screens/MatchesScreen';

const Tab = createBottomTabNavigator();

const AppNavigator = () => {
  return (
    <NavigationContainer>
      <Tab.Navigator
        screenOptions={({route}) => ({
          tabBarIcon: ({focused, color, size}) => {
            let iconName;
            if (route.name === 'Explore') {
              iconName = focused ? 'flame': 'flame-outline';
            } else if (route.name === 'Matches') {
              iconName = focused ? 'heart': 'heart-outline'; 
            }

            return <Ionicons name={iconName} size={size} color={color}/>;
          },
          tabBarActiveTintColor: '#FF5864',
          tabBarInactiveTintColor: 'gray',
          headerShown: false,
        })}
      >
        <Tab.Screen name="Explore" component={PlacesScreen}/>
        <Tab.Screen name="Matches" component={MatchesScreen}/>
      </Tab.Navigator>
    </NavigationContainer>
  );  
}

export default AppNavigator;