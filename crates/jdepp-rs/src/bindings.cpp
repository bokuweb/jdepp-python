#include <algorithm>
#include <atomic>
#include <chrono>
#include <fstream>
#include <mutex>
#include <sstream>
#include <thread>
#include <vector>
#include <iostream>
#include <sstream>
#define OPTPARSE_IMPLEMENTATION
#include "../../../jdepp/optparse.h"
#include "../../../jdepp/pdep.h"
#include "../../../jdepp/io-util.hh"

#ifndef NUM_POS_FIELD
#define NUM_POS_FIELD 4
#endif

class Sentence
{
public:
    Sentence() = default;

    const std::string &str() const
    {
        return _str;
    }

    void set_str(const std::string &s)
    {
        _str = s;
    }

    /*
        const std::vector<PyToken> tokens() const
        {
            return _tokens;
        }

        const std::vector<PyChunk> chunks() const
        {
            return _chunks;
        }

        void set_chunks(const std::vector<PyChunk> &rhs)
        {
            _chunks = rhs;
        }

        void set_chunks(std::vector<PyChunk> &&rhs)
        {
            _chunks = rhs;
        }

        const std::string print(bool prob = false) const
        {
            std::stringstream ss;

            for (const auto &chunk : _chunks)
            {
                ss << chunk.print(prob);
            }

            ss << "EOS\n";

            return ss.str();
        }
        */

private:
    std::string _str;
    // std::vector<PyToken> _tokens;
    // std::vector<PyChunk> _chunks;
};

class Jdepp
{
public:
    const uint32_t kMaxChars = IOBUF_SIZE / 4; // Usually UTF-8 Japanese character is 3-bytes. div by 4 is convervative estimate.
    Jdepp() {}
    Jdepp(const std::string &model_path)
        : _model_path(model_path)
    {
        load_model(_model_path);
    }

    void set_threads(uint32_t nthreads)
    {
        _nthreads = nthreads;
    }

    bool load_model(const std::string &model_path)
    {

        //
        // Curently we must set all parameters in options as (argc, argv), then construct parser.
        //

        // std::cout << "model_path " << model_path << "\n";
        _argv_str.push_back("jdepp");
        //_argv_str.push_back("--verbose");
        //_argv_str.push_back("10");
        _argv_str.push_back("-m");
        _argv_str.push_back(model_path);

        // TODO: args for learner, chunker, depend

        setup_argv();

        pdep::option options(int(_argv.size()), _argv.data(),
                             int(_learner_argv.size()), _learner_argv.data(),
                             int(_chunk_argv.size()), _chunk_argv.data(),
                             int(_depend_argv.size()), _depend_argv.data());

        if (_parser && _parser->model_loaded())
        {
            // discard previous model&instance.
            delete _parser;
        }

        _parser = new pdep::parser(options);
        if (!_parser->load_model())
        { // use setting in argv for model path
          // py::print("Model load failed:", model_path);
        }

        return _parser->model_loaded();
    }

    bool model_loaded() const
    {
        return (_parser && _parser->model_loaded());
    }

    // for internal debugging
    void run()
    {
        _parser->run();
    }

    Sentence *parse_from_postagged(const char *input_postagged, size_t len) const
    {
        Sentence *s = new Sentence();

        if (!_parser->model_loaded())
        {
            // py::print("Model is not yet loaded.");
            return s;
        }

        if (len > IOBUF_SIZE)
        {
            return s;
        }
        const pdep::sentence_t *sent = _parser->parse_from_postagged(input_postagged, len);
        if (!sent)
        {
            // py::print("Failed to parse text from POS tagged string");
            return s;
        }

        // Sentence pysent;

        // We create copy for each chunk/token.
        // This approach is redundunt and not memory-efficient,
        // but this make Python binding easier(we don't need to consider lifetime of Python/C++ object)
        const char *str = sent->print_tostr(pdep::RAW, /* print_prob */ false);

        if (!str)
        {
            // py::print("Failed to get string from C++ sentence_t struct.");
            return s;
        }
        //
        // // Assume single sentence in input text(i.e. one `EOS` line)
        std::string header = "# S-ID: " + std::to_string(1) + "; J.DepP\n";
        s->set_str(header + std::string(str));

        return s;
        // return pysent;
        /*
                       // std::vector<PyChunk> py_chunks;

                       const std::vector<const pdep::chunk_t *>
                           chunks = sent->chunks();
                for (size_t i = 0; i < chunks.size(); i++)
                {
                    const pdep::chunk_t *b = chunks[i];

                    const std::vector<const pdep::chunk_t *> deps = b->dependents();

                    // std::vector<PyChunk> py_deps;

                    for (size_t k = 0; k < deps.size(); k++)
                    {
                        // PyChunk d;
                        // d.id = deps[k]->id;
                        // d.head_id = deps[k]->head_id;
                        // d.head_id_gold = deps[k]->head_id_gold;
                        // d.head_id_cand = deps[k]->head_id_cand;
                        // d.depend_prob = deps[k]->depnd_prob;
                        // d.depend_type_gold = deps[k]->depnd_type_gold;
                        // d.depend_type_cand = deps[k]->depnd_type_cand;
                        //
                        // // Create copy of token info.
                        // std::vector<PyToken> toks;
                        // for (const pdep::token_t *m = deps[k]->mzero(); m <= deps[k]->mlast(); m++)
                        // {
                        //     std::string surface(m->surface, m->length);
                        //     PyToken py_tok(surface, m->feature, m->chunk_start_prob);
                        //
                        //     toks.push_back(py_tok);
                        // }
                        // d.set_tokens(toks);
                        //
                        // py_deps.push_back(d);
                    }

                    //             PyChunk py_chunk;
                    //
                    //             py_chunk.id = b->id;
                    //             py_chunk.head_id = b->head_id;
                    //             py_chunk.head_id_gold = b->head_id_gold;
                    //             py_chunk.head_id_cand = b->head_id_cand;
                    //             py_chunk.depend_prob = b->depnd_prob;
                    //             py_chunk.depend_type_gold = b->depnd_type_gold;
                    //             py_chunk.depend_type_cand = b->depnd_type_cand;
                    //
                    //             py_chunk.set_dependents(py_deps);
                    //
                    //             std::vector<PyToken> toks;
                    //             for (const pdep::token_t *m = b->mzero(); m <= b->mlast(); m++)
                    //             {
                    //                 std::string surface(m->surface, m->length);
                    //                 PyToken py_tok(surface, m->feature, m->chunk_start_prob);
                    //
                    //                 toks.push_back(py_tok);
                    //             }
                    //             py_chunk.set_tokens(toks);
                    //
                    //             py_chunks.push_back(py_chunk);
                    //         }
                    //
                    //         pysent.set_chunks(std::move(py_chunks));

                    return pysent;
                }
                */
    };

private:
    void setup_argv()
    {
        _argv.clear();
        for (auto &v : _argv_str)
        {
            _argv.push_back(const_cast<char *>(v.c_str()));
        }
        _argv.push_back(nullptr); // must add 'nullptr' at the end, otherwise out-of-bounds access will happen in optparse

        for (auto &v : _learner_argv_str)
        {
            _learner_argv.push_back(const_cast<char *>(v.c_str()));
        }
        _learner_argv.push_back(nullptr);

        for (auto &v : _chunk_argv_str)
        {
            _chunk_argv.push_back(const_cast<char *>(v.c_str()));
        }
        _chunk_argv.push_back(nullptr);

        for (auto &v : _depend_argv_str)
        {
            _depend_argv.push_back(const_cast<char *>(v.c_str()));
        }
        _depend_argv.push_back(nullptr);
    }

    uint32_t _nthreads{0}; // 0 = use all cores
    std::string _model_path;
    pdep::parser *_parser{nullptr};

    std::vector<char *> _argv;
    std::vector<std::string> _argv_str;

    std::vector<char *> _learner_argv;
    std::vector<std::string> _learner_argv_str;

    std::vector<char *> _chunk_argv;
    std::vector<std::string> _chunk_argv_str;

    std::vector<char *> _depend_argv;
    std::vector<std::string> _depend_argv_str;
};

extern "C"
{
    Jdepp *jdepp_create(const char *model_path)
    {
        return new Jdepp(std::string(model_path));
    }

    void jdepp_destroy(Jdepp *instance)
    {
        delete instance;
    }

    bool jdepp_model_loaded(const Jdepp *instance)
    {
        return instance->model_loaded();
    }

    Sentence *parse_from_postagged(const Jdepp *instance, const char *input_postagged, size_t len)
    {
        return instance->parse_from_postagged(input_postagged, len);
    }

    const std::string *sentence_str(const Sentence *instance)
    {
        return &instance->str();
    }
}