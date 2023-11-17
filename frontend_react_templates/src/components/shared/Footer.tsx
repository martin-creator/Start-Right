type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
}

const Footer: React.FC<Props> = ({setCurrentPage, currentPage}) => {
  const pages = ["home_page", "quiz_page", "results_page"];
  const colors = ["#32a852", "#0fa0d1", "#d10fcb"];

  return (
    <div className="fixed bottom-0 w-full bg-gray-800 text-white p-4">
      <div className="flex justify-around">
        {pages.map((page, index) => (
          <button
            key={index}
            onClick={() => setCurrentPage(page)}
            className={`p-2 ${currentPage === page ? colors[index] : 'text-gray-400'}`}
          >
            {page.replace('_', ' ')}
          </button>
        ))}
      </div>
    </div>
  );
}

export default Footer;