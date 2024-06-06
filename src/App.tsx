import { useMutation } from '@apollo/client';
import { CreateNoteDocument } from '@gql/graphql';
import './App.css';

function App() {
    const [createNote] = useMutation(CreateNoteDocument);

    const handleClick = async () => {
        await createNote({
            variables: {
                title: 'another note',
            },
        });
    };

    return (
        <div className="pt-6">
            <button onClick={handleClick}>Click Me</button>
        </div>
    );
}

export default App;
