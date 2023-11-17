import React, { useEffect, useState } from 'react';
import axios from 'axios';
import 'tailwindcss/tailwind.css';

function MasterPage() {
  const [meetings, setMeetings] = useState([]);
  const [meeting, setMeeting] = useState(null);
  const [error, setError] = useState(null);

  const fetchMeetings = async () => {
    try {
      const response:any = await axios.get('/api/v1/meetings');
      setMeetings(response.data);
    } catch (e:any) {
      setError(e.message);
    }
  };

  const fetchMeeting = async (id:any) => {
    try {
      const response:any = await axios.get(`/api/v1/meetings/${id}`);
      setMeeting(response.data);
    } catch (e:any) {
      setError(e.message);
    }
  };

  const createMeeting = async (meetingData:any) => {
    try {
      await axios.post('/api/v1/meetings', meetingData);
      fetchMeetings();
    } catch (e:any) {
      setError(e.message);
    }
  };

  const updateMeeting = async (id:any, meetingData:any) => {
    try {
      await axios.put(`/api/v1/meetings/${id}`, meetingData);
      fetchMeetings();
    } catch (e:any) {
      setError(e.message);
    }
  };

  const deleteMeeting = async (id:any) => {
    try {
      await axios.delete(`/api/v1/meetings/${id}`);
      fetchMeetings();
    } catch (e) {
      setError(e.message);
    }
  };

  useEffect(() => {
    fetchMeetings();
  }, []);

  return (
    <div className="w-full h-full bg-gray-100">
      <section className="flex flex-col items-center justify-center p-4">
        <div className="meeting_details_section flex flex-col items-center justify-center p-4 bg-white shadow-lg rounded-lg w-3/4">
            <h1 className="text-2xl font-bold text-blue-600">Meeting Details</h1>
            <p className="text-lg text-gray-600">Here you can find all the details about the specific meeting, including its name and whether it has been completed.</p>
        </div>
        <div className="update_meeting_section flex flex-col items-center justify-center p-4 bg-white shadow-lg rounded-lg w-3/4 mt-4">
            <h1 className="text-2xl font-bold text-blue-600">Update Meeting</h1>
            <p className="text-lg text-gray-600">In this section, you can update the details of the meeting as per your requirements.</p>
        </div>
        <div className="delete_meeting_section flex flex-col items-center justify-center p-4 bg-white shadow-lg rounded-lg w-3/4 mt-4">
            <h1 className="text-2xl font-bold text-blue-600">Delete Meeting</h1>
            <p className="text-lg text-gray-600">If you no longer need the meeting, you can delete it in this section.</p>
        </div>
      </section>
    </div>
  );
}

export default MasterPage;