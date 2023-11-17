type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
}

const Footer: React.FC<Props> = ({setCurrentPage, currentPage}) => {
  const pages = ['home_page', 'meeting_page'];
  const colors = ['#32a852', '#0fa0d1', '#d10fcb'];

  return (
    <div className="fixed bottom-0 w-full bg-gray-200 p-4 flex justify-around items-center">
      {pages.map((page, index) => (
        <button 
          key={index} 
          onClick={() => setCurrentPage(page)}
          className={`p-2 rounded ${currentPage === page ? colors[index % colors.length] : 'text-gray-500'}`}
        >
          {page}
        </button>
      ))}
    </div>
  );
}

export default Footer;