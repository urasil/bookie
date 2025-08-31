import React, { useEffect, useState } from 'react';
import type { Place } from '../types';
import { getMatches } from '../api';

const Matches: React.FC = () => {
  const [matches, setMatches] = useState<Place[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchMatches = async () => {
      try {
        const data = await getMatches();
        setMatches(data);
      } catch (err) {
        setError('Failed to fetch matches.');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };
    fetchMatches();
  }, []);

  if (loading) {
    return <div>Loading matches...</div>;
  }

  if (error) {
    return <div>Error: {error}</div>;
  }

  if (matches.length === 0) {
    return <div>No matched properties yet. Keep swiping!</div>;
  }

  return (
    <div className="matches-container">
      <h2>Your Matched Properties</h2>
      <div className="matches-list">
        {matches.map((place) => (
          <div key={place.id} className="match-card">
            <img src={place.image} alt={place.name} className="match-image" />
            <h3>{place.name}</h3>
            <p><strong>Location:</strong> {place.location}</p>
            <p><strong>Price:</strong> ${place.price}/night</p>
            <p>{place.desc}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Matches;
