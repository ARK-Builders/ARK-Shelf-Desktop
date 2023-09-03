import {
  AppBar,
  Button,
  Card,
  CardContent,
  Grid,
  List,
  Toolbar,
  Container,
  Typography,
  TextField,
  Pagination,
  IconButton,
  ButtonGroup,
  Tooltip,
} from "@mui/material";
import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api";
import { ToastContainer, toast, Slide } from "react-toastify";
import { useForm, SubmitHandler, Controller } from "react-hook-form";
import "react-toastify/dist/ReactToastify.css";
import {
  CalendarMonth,
  FormatListBulleted,
  SortByAlpha,
} from "@mui/icons-material";
import { LinkListItem } from "./components/LinkCard";
import { LinkInfo, LinkScoreMap, OpenGraph, SortMode } from "./types";

const Home = () => {
  const [mode, setMode] = useState<SortMode>("normal");

  const [linkInfos, setLinkInfos] = useState<LinkInfo[]>([]);
  const [scores, setScores] = useState<LinkScoreMap[]>([]);

  const {
    handleSubmit,
    reset,
    setValue,
    getValues,
    control,
    formState: { dirtyFields, isDirty },
  } = useForm<LinkInfo>();
  const [page, setPage] = useState(0);
  const itemPerPage = 5;
  const pageCount = Math.ceil(linkInfos.length / itemPerPage);
  const createLink: SubmitHandler<LinkInfo> = async (data) => {
    data.desc = data.desc ?? "";
    console.log("Submit", { data });
    await invoke("create_link", {
      ...data,
    });
    refreshInfo();
    toast("Link created!");
    reset();
    // Forced refreash view to ensure updated list.
  };

  // Refresh all info.
  const refreshInfo = useCallback(async () => {
    const names = await invoke<string[]>("read_link_list");
    const linkPromises = names.map(async (name) => {
      const link = await invoke<Omit<LinkInfo, "score" | "name">>("read_link", {
        name,
      });
      return {
        ...link,
        score: 0,
        name,
      };
    });
    const links = await Promise.all(linkPromises);

    let newScores: LinkScoreMap[] = [];
    try {
      newScores = await invoke<LinkScoreMap[]>("get_scores");
    } finally {
      setScores(newScores);
    }
    // Sort and push link infos
    links.sort((a, b) => {
      switch (mode) {
        case "normal":
          return a.title?.localeCompare(b.title ?? "") ?? 0;
        case "date":
          return (
            b.created_time?.secs_since_epoch ??
            0 - (a.created_time?.secs_since_epoch ?? 0)
          );
        case "score": {
          const itemA = newScores.find((v) => v.name === a.name)?.value ?? 0;
          const itemB = newScores.find((v) => v.name === b.name)?.value ?? 0;
          return itemB - itemA;
        }
        default:
          return 0;
      }
    });
    setLinkInfos(links);
  }, [mode]);

  useEffect(() => {
    refreshInfo();
  }, [refreshInfo]);

  useEffect(() => {
    console.log("Scores", { scores });
  }, [scores]);

  const LinkList = () => {
    return (
      <List>
        {linkInfos
          .map((val, idx) => {
            return (
              <div id={val.title} key={val.title}>
                <LinkListItem
                  link={val}
                  index={idx}
                  refresh={refreshInfo}
                  mode={mode}
                  scores={scores}
                  setScores={setScores}
                />
              </div>
            );
          })
          .slice(itemPerPage * page, itemPerPage * (page + 1))}
      </List>
    );
  };

  return (
    <>
      <AppBar position="fixed">
        <Toolbar>
          <Typography
            variant="h6"
            sx={{
              flexGrow: 1,
            }}
          >
            ARK Shelf
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Container
        sx={{
          mt: 2,
        }}
      >
        <Grid container spacing={8}>
          <Grid item xs={8}>
            <Card>
              <CardContent>
                <LinkList />

                <Pagination
                  count={pageCount === 0 ? 1 : pageCount}
                  page={page + 1}
                  onChange={(_, page) => {
                    // mui pages are started from 1, against to zero-based index array
                    setPage(page - 1);
                  }}
                  showFirstButton
                  showLastButton
                />
              </CardContent>
            </Card>
          </Grid>
          <Grid item xs={4}>
            <Grid item>
              <ButtonGroup variant="outlined">
                <Tooltip key="alphabet" title={"Sorting By Alphabet"}>
                  <IconButton
                    onClick={() => {
                      setMode("normal");
                    }}
                  >
                    <SortByAlpha />
                  </IconButton>
                </Tooltip>

                <Tooltip key="date" title={"Sorting By Date"}>
                  <IconButton
                    onClick={() => {
                      setMode("date");
                    }}
                  >
                    <CalendarMonth />
                  </IconButton>
                </Tooltip>
                <Tooltip key="score" title={"Sorting By Score"}>
                  <IconButton
                    onClick={() => {
                      setMode("score");
                    }}
                  >
                    <FormatListBulleted />
                  </IconButton>
                </Tooltip>
              </ButtonGroup>
            </Grid>
            <form onSubmit={handleSubmit(createLink)}>
              <Controller
                control={control}
                name="url"
                render={({ field: { value, onChange, onBlur } }) => (
                  <TextField
                    fullWidth
                    label="URL"
                    margin="normal"
                    required={true}
                    onChange={onChange}
                    onBlur={onBlur}
                    value={value ?? ""}
                  />
                )}
              />

              <Controller
                control={control}
                name="title"
                render={({ field: { value, onChange, onBlur } }) => (
                  <TextField
                    fullWidth
                    label="Title"
                    margin="normal"
                    required={true}
                    onChange={onChange}
                    onBlur={onBlur}
                    value={value ?? ""}
                  />
                )}
              />
              <Controller
                control={control}
                name="desc"
                render={({ field: { value, onChange, onBlur } }) => (
                  <TextField
                    fullWidth
                    label="Description (optional)"
                    margin="normal"
                    required={false}
                    onChange={onChange}
                    onBlur={onBlur}
                    value={value ?? ""}
                  />
                )}
              />

              <Button type="submit">Create</Button>
              <Button
                onClick={() => {
                  if (isDirty && dirtyFields.url) {
                    let url = getValues("url");
                    invoke("generate_link_preview", {
                      url: url.toString(),
                    })
                      .then((val) => {
                        let data = val as OpenGraph;
                        console.log(data);
                        if (!data.title) {
                          toast("Failed to fetch website data.");
                        }
                        setValue("title", data.title, { shouldDirty: true });
                        setValue("desc", data.description, {
                          shouldDirty: true,
                        });
                        console.log(dirtyFields);
                      })
                      .catch((e) => console.log(e));
                  }
                }}
                color="error"
              >
                Auto Filled
              </Button>
            </form>
          </Grid>
        </Grid>
      </Container>
      <ToastContainer
        position="bottom-right"
        autoClose={1000}
        hideProgressBar
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        transition={Slide}
        theme="dark"
      />
    </>
  );
};
export default Home;
