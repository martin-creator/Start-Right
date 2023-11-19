import React, { useEffect, useState } from 'react';
import axios from 'axios';
import 'tailwindcss/tailwind.css';

function MasterPage() {
  const [stations, setStations] = useState([]);
  const [station, setStation] = useState(null);
  const [error, setError] = useState(null);

  const fetchStations = async () => {
    try {
      const response:any = await axios.get('/api/v1/stations');
      setStations(response.data);
    } catch (e:any) {
      setError(e.message);
    }
  };

  const fetchStation = async (id:any) => {
    try {
      const response:any = await axios.get(`/api/v1/stations/${id}`);
      setStation(response.data);
    } catch (e:any) {
      setError(e.message);
    }
  };

  const createStation = async (stationData:any) => {
    try {
      await axios.post('/api/v1/stations', stationData);
      fetchStations();
    } catch (e:any) {
      setError(e.message);
    }
  };

  const updateStation = async (id:any, stationData:any) => {
    try {
      await axios.put(`/api/v1/stations/${id}`, stationData);
      fetchStations();
    } catch (e:any) {
      setError(e.message);
    }
  };

  const deleteStation = async (id:any) => {
    try {
      await axios.delete(`/api/v1/stations/${id}`);
      fetchStations();
    } catch (e) {
      setError(e.message);
    }
  };

  useEffect(() => {
    fetchStations();
  }, []);

  return (
    <div className="w-full h-full bg-gray-100">
      <section className="flex flex-col items-center justify-center p-4">
        <div className="station_details_section flex flex-col items-center justify-center p-4 bg-white shadow-lg rounded-lg w-3/4">
            <h1 className="text-2xl font-bold text-blue-600">Station Details</h1>
            <p className="text-lg text-gray-600">Here you can find all the details about the specific station.</p>
        </div>
        <div className="update_station_section flex flex-col items-center justify-center p-4 bg-white shadow-lg rounded-lg w-3/4 mt-4">
            <h1 className="text-2xl font-bold text-blue-600">Update Station</h1>
            <p className="text-lg text-gray-600">In this section, you can update the details of the station as per your requirements.</p>
        </div>
        <div className="delete_station_section flex flex-col items-center justify-center p-4 bg-white shadow-lg rounded-lg w-3/4 mt-4">
            <h1 className="text-2xl font-bold text-blue-600">Delete Station</h1>
            <p className="text-lg text-gray-600">If you no longer need the station, you can delete it in this section.</p>
        </div>
      </section>
      <section className="flex flex-col items-center justify-center">
        <div className="w-full md:w-2/3 lg:w-1/2">
            <h1 className="text-2xl font-bold text-center">Dashboard</h1>
            <div className="mt-4">
                <h2 className="text-xl font-semibold">Stations</h2>
                <div className="mt-2">
                    <p>A list of all stations</p>
                    <div id="stations_section" className="flex flex-wrap justify-center">
                        {/* Fetch and display stations from http://localhost:8080/stations */}
                    </div>
                </div>
                <div className="mt-4">
                    <h2 className="text-xl font-semibold">Create Station</h2>
                    <div className="mt-2">
                        <p>A form to create new stations</p>
                        <form id="create_station_section" className="flex flex-col">
                            {/* Form to create new stations, POST to http://localhost:8080/stations */}
                        </form>
                    </div>
                </div>
                <div className="mt-4">
                    <h2 className="text-xl font-semibold">Update Station</h2>
                    <div className="mt-2">
                        <p>A form to update existing stations</p>
                        <form id="update_station_section" className="flex flex-col">
                            {/* Form to update existing stations, PUT to http://localhost:8080/stations/:id */}
                        </form>
                    </div>
                </div>
                <div className="mt-4">
                    <h2 className="text-xl font-semibold">Delete Station</h2>
                    <div className="mt-2">
                        <p>A button to delete stations</p>
                        <button id="delete_station_section" className="px-4 py-2 bg-red-500 text-white rounded">
                            {/* Button to delete stations, DELETE to http://localhost:8080/stations/:id */}
                            Delete Station
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </section>
    <section className="flex flex-col md:flex-row">
        <div className="flex flex-col md:w-1/2 p-4">
            <h2 className="text-2xl font-bold mb-4">Station Details</h2>
            <p className="mb-2">Here you can find detailed information about a specific station, including its current status and schedule. This information is updated in real time to ensure accuracy and reliability.</p>
            <button className="bg-blue-500 text-white px-4 py-2 rounded">View Station Details</button>
        </div>
        <div className="flex flex-col md:w-1/2 p-4">
            <h2 className="text-2xl font-bold mb-4">Station Updates</h2>
            <p className="mb-2">Stay informed with real-time updates related to the station, such as delays or changes in schedule. These updates are provided directly from the station control center.</p>
            <button className="bg-blue-500 text-white px-4 py-2 rounded">View Station Updates</button>
        </div>
        <div className="flex flex-col md:w-full p-4">
            <h2 className="text-2xl font-bold mb-4">Feedback</h2>
            <p className="mb-2">We value your feedback. If you have any issues or suggestions about the station or its services, please let us know. Your feedback helps us improve our services.</p>
            <button className="bg-blue-500 text-white px-4 py-2 rounded">Leave Feedback</button>
        </div>
    </section>
    <section className="flex flex-col items-center justify-center p-4">
        <div className="w-full max-w-xs">
            <h2 className="text-center text-2xl font-bold mb-4">Create Station</h2>
            <form className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                <div className="mb-4">
                    <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="stationName">
                        Station Name
                    </label>
                    <input className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="stationName" type="text" placeholder="Station Name"/>
                </div>
                <div className="mb-4">
                    <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="stationDetails">
                        Station Details
                    </label>
                    <textarea className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="stationDetails" placeholder="Station Details"></textarea>
                </div>
                <div className="flex items-center justify-between">
                    <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">
                        Create Station
                    </button>
                </div>
            </form>
        </div>
        <div className="w-full max-w-xs">
            <h2 className="text-center text-2xl font-bold mb-4">Delete Station</h2>
            <button className="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">
                Delete Station
            </button>
        </div>
        <div className="w-full max-w-xs">
            <h2 className="text-center text-2xl font-bold mb-4">Station Details</h2>
            <div className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                <div className="mb-4">
                    <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="stationName">
                        Station Name
                    </label>
                    <p className="text-gray-700 text-base" id="stationName">Station Name</p>
                </div>
                <div className="mb-4">
                    <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="stationDetails">
                        Station Details
                    </label>
                    <p className="text-gray-700 text-base" id="stationDetails">Station Details</p>
                </div>
            </div>
        </div>
        <div className="w-full max-w-xs">
            <h2 className="text-center text-2xl font-bold mb-4">Update Station</h2>
            <form className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                <div className="mb-4">
                    <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="stationName">
                        Station Name
                    </label>
                    <input className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="stationName" type="text" placeholder="Station Name"/>
                </div>
                <div className="mb-4">
                    <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="stationDetails">
                        Station Details
                    </label>
                    <textarea className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="stationDetails" placeholder="Station Details"></textarea>
                </div>
                <div className="flex items-center justify-between">
                    <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">
                        Update Station
                    </button>
                </div>
            </form>
        </div>
    </section>
    </div>
  );
}

export default MasterPage;