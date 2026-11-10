// Loaded only from a copy in the disposable runner's owned fixture directory.
// These callbacks expose discovery shapes; none perform authentication/search.
export default {
  id: "opencoders.discovery.fixture",
  async setup(ctx) {
    await ctx.integration.transform((editor) => {
      for (const [id, name] of [
        ["opencoders-discovery", "Owned discovery integration"],
        ["opencoders-discovery-denied", "Denied discovery integration"],
      ]) {
        editor.update(id, (ref) => {
          ref.name = name;
          ref.metadata = { fixture: true };
        });
      }
      const integrationID = "opencoders-discovery";
      const form = [{ type: "string", key: "fixture", title: "Fixture", default: "owned" }];
      editor.method.update({ integrationID, method: {type:"oauth",id:"fixture-oauth",label:"Fixture OAuth",form},
        async authorize() { throw new Error("Discovery fixture must not authorize"); } });
      for (const method of [
        {type:"command",id:"fixture-command",label:"Fixture command",command:["/usr/bin/false"]},
        {type:"key",label:"Fixture key",form},
        {type:"external",id:"fixture-external",label:"Fixture external",form},
        {type:"env",names:["OPENCODERS_DISCOVERY_PRESENT","OPENCODERS_DISCOVERY_MISSING"]},
      ]) editor.method.update({integrationID,method});
    });
    await ctx.integration.connection.status({
      integrationID:"opencoders-discovery",
      connection:{type:"env",name:"OPENCODERS_DISCOVERY_PRESENT"},
      status:{status:"needs_auth",message:"Owned fixture status",url:"https://example.invalid/fixture"},
    });
    await ctx.websearch.transform((editor) => {
      for (const [id,name] of [["fixture-z","Zulu fixture search"],["fixture-a","Alpha fixture search"]]) {
        editor.add({id,name,async execute() {throw new Error("Discovery fixture must not search");}});
      }
    });
  },
};
