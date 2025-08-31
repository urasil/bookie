import React, { useEffect, useState } from 'react';
import type { Place } from '../types';
import { getPlaces, likePlace } from '../api';

const Home: React.FC = () => {
  const [places, setPlaces] = useState<Place[]>([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchPlaces = async () => {
      try {
        const data = await getPlaces();
        setPlaces(data);
      } catch (err) {
        setError('Failed to fetch places.');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };
    fetchPlaces();
  }, []);

  const handleLike = async () => {
    if (places.length > 0 && currentIndex < places.length) {
      const currentPlace = places[currentIndex];
      try {
        await likePlace(currentPlace.id);
        setCurrentIndex((prevIndex) => prevIndex + 1);
      } catch (err) {
        setError('Failed to like place.');
        console.error(err);
      }
    }
  };

  const handleDislike = () => {
    if (places.length > 0 && currentIndex < places.length) {
      setCurrentIndex((prevIndex) => prevIndex + 1);
    }
  };

  if (loading) {
    return <div>Loading properties...</div>;
  }

  if (error) {
    return <div>Error: {error}</div>;
  }

  if (places.length === 0 || currentIndex >= places.length) {
    return <div>No more properties to display. Check back later!</div>;
  }

  const currentPlace = places[currentIndex];

  return (
    <div className="home-container">
      <h2>Discover New Places</h2>
      <div className="place-card">
        <img src={currentPlace.image} alt={currentPlace.name} className="place-image" />
        <h3>{currentPlace.name}</h3>
        <p><strong>Location:</strong> {currentPlace.location}</p>
        <p><strong>Price:</strong> ${currentPlace.price}/night</p>
        <p>{currentPlace.desc}</p>
        <div className="actions">
          <button onClick={handleDislike} className="dislike-button">Dislike</button>
          <button onClick={handleLike} className="like-button">Like</button>
        </div>
      </div>
    </div>
  );
};

export default Home;
