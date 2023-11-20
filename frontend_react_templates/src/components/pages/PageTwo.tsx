import React, { useEffect, useState } from 'react';
import axios from 'axios';
import 'tailwindcss/tailwind.css';

function NutritionPage() {
  const [nutritionData, setNutritionData] = useState([]);
  const [nutritionItem, setNutritionItem] = useState(null);
  const [error, setError] = useState(null);

  const fetchNutritionData = async () => {
    try {
      const response:any = await axios.get('/api/v1/nutrition');
      setNutritionData(response.data);
    } catch (e:any) {
      setError(e.message);
    }
  };

  const fetchNutritionItem = async (id:any) => {
    try {
      const response:any = await axios.get(`/api/v1/nutrition/${id}`);
      setNutritionItem(response.data);
    } catch (e:any) {
      setError(e.message);
    }
  };

  const createNutritionItem = async (nutritionData:any) => {
    try {
      await axios.post('/api/v1/nutrition', nutritionData);
      fetchNutritionData();
    } catch (e:any) {
      setError(e.message);
    }
  };

  const updateNutritionItem = async (id:any, nutritionData:any) => {
    try {
      await axios.put(`/api/v1/nutrition/${id}`, nutritionData);
      fetchNutritionData();
    } catch (e:any) {
      setError(e.message);
    }
  };

  const deleteNutritionItem = async (id:any) => {
    try {
      await axios.delete(`/api/v1/nutrition/${id}`);
      fetchNutritionData();
    } catch (e:any) {
      setError(e.message);
    }
  };

  useEffect(() => {
    fetchNutritionData();
  }, []);

  return (
    <div className="w-full h-full bg-gray-100 p-8">
      <section className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
        <div className="user_info_section flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-md">
            <h2 className="text-2xl font-bold mb-2">User Information</h2>
            <p className="text-lg text-gray-600">Here you can view your personal information and nutrition requirements.</p>
        </div>
        <div className="update_info_section flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-md">
            <h2 className="text-2xl font-bold mb-2">Update Information</h2>
            <p className="text-lg text-gray-600">Update your nutrition requirements, allergies, and other personal information here.</p>
        </div>
        <div className="nutrition_tracking_section flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-md">
            <h2 className="text-2xl font-bold mb-2">Nutrition Tracking</h2>
            <p className="text-lg text-gray-600">Input and track your daily nutrition intake and monitor your allergies here.</p>
        </div>
        <div className="logout_section flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-md">
            <h2 className="text-2xl font-bold mb-2">Logout</h2>
            <p className="text-lg text-gray-600">Click the button below to securely log out of your account.</p>
        </div>
      </section>
    </div>
  );
}

export default NutritionPage;