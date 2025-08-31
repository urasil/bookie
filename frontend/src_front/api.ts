import type { Place } from './types';

const API_BASE_URL = 'http://localhost:3000'; // Replace with your backend URL

export const getPlaces = async (): Promise<Place[]> => {
  const response = await fetch(`${API_BASE_URL}/places`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const data: Place[] = await response.json();
  return data;
};

export const likePlace = async (id: string): Promise<void> => {
  const response = await fetch(`${API_BASE_URL}/like/${id}`, {
    method: 'POST',
  });
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
};

export const getMatches = async (): Promise<Place[]> => {
  const response = await fetch(`${API_BASE_URL}/matches`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const data: Place[] = await response.json();
  return data;
};
